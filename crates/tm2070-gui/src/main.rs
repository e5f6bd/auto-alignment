use std::{any::TypeId, thread::sleep, time::Duration};

use iced::{
    futures::SinkExt,
    widget::{
        canvas::{Cache, Geometry, Path, Program, Stroke},
        Canvas,
    },
    Application, Color, Command, Length, Point, Rectangle, Renderer, Settings, Theme,
};
use log::trace;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() -> iced::Result {
    env_logger::builder().format_timestamp_nanos().init();
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    points: Vec<f64>,
    waveform_frame_cache: Cache,
}
impl Application for App {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Auto collimator monitor".to_owned()
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        match _message {
            Message::DataPoint(point) => {
                self.points.push(point);
                self.waveform_frame_cache.clear();
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        struct SubscriptionId;
        iced::subscription::channel(TypeId::of::<SubscriptionId>(), 100, |mut tx| async move {
            // trace!("Subscription started");
            let mut rng = StdRng::from_entropy();
            loop {
                // trace!("Point generated");
                for _ in 0..10 {
                    let _ = tx
                        .send(Message::DataPoint(rng.gen_range(100. ..150.)))
                        .await;
                }
                sleep(Duration::from_secs_f64(1. / 60.));
            }
        })
    }
}

impl Program<Message> for App {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self
            .waveform_frame_cache
            .draw(renderer, bounds.size(), |frame| {
                trace!("Draw");

                // Background
                frame.fill_rectangle(bounds.position(), bounds.size(), Color::BLACK);

                let path = Path::new(|builder| {
                    let mut points = (self.points.iter().enumerate())
                        .map(|(i, &point)| Point::new(i as f32 / 10., point as _));
                    if let Some(point) = points.by_ref().next() {
                        builder.move_to(point);
                    };
                    for point in points {
                        builder.line_to(point);
                    }
                });
                frame.stroke(&path, Stroke::default().with_color(Color::from_rgba(1., 1., 1., 0.01)));
            });
        vec![geometry]
    }
}

#[derive(Debug)]
enum Message {
    DataPoint(f64),
}
