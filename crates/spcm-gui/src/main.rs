use std::{array, future::pending, sync::mpsc, thread::spawn};

use anyhow::Context;
use iced::{
    futures::{channel::mpsc as mpsc_f, SinkExt, StreamExt},
    theme,
    widget::{button, column, horizontal_space, row, slider, text, text_input},
    Alignment, Application, Color, Command, Renderer, Settings, Theme,
};
use log::error;
use regex::Regex;
use spcm::{FREQUENCY_STEP, PHASE_STEP};
use text_input_theme::CustomBackgroundTextInput;

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    App::run(Settings {
        ..Default::default()
    })?;
    Ok(())
}

struct App {
    dds_connector: Option<mpsc_f::Sender<String>>,
    address: String,
    connected: bool,
    channel_states: [ChannelState; 4],
    error_message: String,
}
struct ChannelState {
    amplitude: String,
    frequency_text: String,
    frequency_editing: bool,
    frequency: u32,
    phase: u16,
}
impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new((): ()) -> (Self, Command<Message>) {
        (
            App {
                dds_connector: None,
                address: "/dev/spcm0".to_owned(),
                connected: false,
                channel_states: array::from_fn(|_| ChannelState {
                    amplitude: "100".into(),
                    frequency_text: "1 000 000.0".into(),
                    frequency_editing: false,
                    frequency: 3435974,
                    phase: 0,
                }),
                error_message: "".to_owned(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Spectrum DDS".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Nop => {}
            Message::EstablishSubscriptionConnection(sender) => self.dds_connector = Some(sender),
            Message::AddressChanged(x) => self.address = x,
            Message::Connect => {
                let Some(dds_connector) = &mut self.dds_connector else {
                    self.report_error("Waiting for the DDS worker to start...".to_owned());
                    return Command::none();
                };
                return Command::perform(dds_connector.send(self.address.clone()), |_| {
                    Message::Nop
                });
            }
            Message::AmplitudeChanged(i, x) => self.channel_states[i].amplitude = x,
            Message::FrequencyTextChanged(i, x) => {
                let c = &mut self.channel_states[i];
                c.frequency_text = x;
                c.frequency_editing = true;
            }
            #[allow(clippy::inconsistent_digit_grouping)]
            Message::SubmitFrequency(i) => {
                let c = &mut self.channel_states[i];
                match c.parse_frequency() {
                    Ok(frequency) => {
                        c.frequency = frequency;
                        c.frequency_text = {
                            let val = (frequency as f64 * FREQUENCY_STEP * 10.).round() as u64;
                            dbg!(val);
                            let g = val / 1_000_000_000_0 % 1000;
                            let m = val / 1_000_000_0 % 1000;
                            let k = val / 1_000_0 % 1000;
                            let o = val / 10 % 1000;
                            let f = val % 10;
                            dbg!(g, m, k, o, f);
                            if g > 0 {
                                format!("{g} {m:03} {k:03} {o:03}.{f:01}")
                            } else if m > 0 {
                                format!("{m} {k:03} {o:03}.{f:01}")
                            } else if k > 0 {
                                format!("{k} {o:03}.{f:01}")
                            } else {
                                format!("{o}.{f:01}")
                            }
                        };
                        c.frequency_editing = false;
                        // Send change to DDS
                    }
                    Err(e) => {
                        self.report_error(format!("{e:#}"));
                    }
                }
            }
            Message::PhaseChanged(i, x) => self.channel_states[i].phase = x,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Message, Theme, Renderer> {
        type E<'a> = iced::Element<'a, Message, Theme, Renderer>;
        let address_row = {
            let address_input = {
                let mut t = text_input("", &self.address);
                if !self.connected {
                    t = t
                        .on_input(Message::AddressChanged)
                        .on_submit(Message::Connect);
                }
                t
            };
            let connect_button =
                button("Connect").on_press_maybe((!self.connected).then_some(Message::Connect));
            row![address_input, connect_button]
        };
        let mut column = column![address_row];
        for (i, channel) in self.channel_states.iter().enumerate() {
            let label = text(&format!("Channel {i}"));
            let amplitude_label = text("Amplitude:");
            let amplitude_input = {
                let mut t = text_input("80-2500", &channel.amplitude);
                if !self.connected {
                    t = t.on_input(move |x| Message::AmplitudeChanged(i, x));
                }
                t
            };
            let frequency_label = text("Frequency:");
            let frequency_input = {
                let mut t =
                    text_input("", &channel.frequency_text).on_submit(Message::SubmitFrequency(i));
                if self.connected {
                    t = t.on_input(move |x| Message::FrequencyTextChanged(i, x));
                }
                {
                    let color = channel
                        .frequency_editing
                        .then(|| Color::from_rgb(1.0, 1.0, 0.8));
                    // Err(_) => Color::from_rgb(1.0, 0.8, 0.8),
                    t = t.style(theme::TextInput::Custom(Box::new(
                        CustomBackgroundTextInput(self.theme(), color),
                    )));
                }
                t
            };
            let frequency_text = text(&format!("0x{:08x}", channel.frequency));
            let phase_label = text(&format!(
                "Phase: {:.02}°",
                channel.phase as f64 * PHASE_STEP
            ));
            let phase_slider = slider(0..=4095, channel.phase, move |x| {
                Message::PhaseChanged(i, x)
            });
            let space = || horizontal_space().width(10);
            let row: E = row![
                label,
                space(),
                amplitude_label,
                amplitude_input,
                space(),
                frequency_label,
                frequency_input,
                frequency_text,
                space(),
                phase_label,
                phase_slider,
            ]
            .align_items(Alignment::Center)
            .into();
            column = column.push(row);
        }
        {
            let error = text(&self.error_message);
            column = column.push(error);
        }
        column.into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::channel((), 100, |mut tx| async move {
            let (tx_app, mut rx) = mpsc_f::channel(100);
            if let Err(e) = tx
                .send(Message::EstablishSubscriptionConnection(tx_app))
                .await
            {
                error!("Failed to establish subscription connection");
                pending().await
            }
            while let Some(address) = rx.next().await {
                let tx = tx.clone();
                spawn(move || pollster::block_on(dds_worker(tx, address)));
            }
            error!("End of stream");
            pending().await
        })
    }
}

enum DdsWorkerMessage {
    // Connect(String),
}

async fn dds_worker(mut tx: mpsc_f::Sender<Message>, address: String) {}

impl App {
    fn report_error(&mut self, message: String) {
        self.error_message = message
    }
}

impl ChannelState {
    fn parse_frequency(&self) -> anyhow::Result<u32> {
        let value = {
            let captures = Regex::new(
                r"(?x)^\s*
                   (?P<num>[0-9][0-9\x20]*(\.[0-9\x20]*)?[0-9])
                   \s*
                   (?P<suffix>[kKmM]?)
                   \s*$",
            )
            .unwrap()
            .captures(&self.frequency_text)
            .context("Invalid frequency syntax")?;
            let f: f64 = dbg!(&captures)
                .name("num")
                .unwrap()
                .as_str()
                .replace(' ', "")
                .parse()
                .context("Failed to parse frequency into f64")?;
            let suffix = match captures.name("suffix").unwrap().as_str() {
                "k" | "K" => 1e3,
                "m" | "M" => 1e6,
                _ => 1.,
            };
            f * suffix
        };
        dbg!(value);
        // u32 range \subset i64 range.
        // First convert to i64, then to u32, to check out-of-range
        (dbg!(value / FREQUENCY_STEP).round() as i64)
            .try_into()
            .context("Value out of range")
    }
}

#[derive(Clone, Debug)]
enum Message {
    Nop,
    EstablishSubscriptionConnection(mpsc_f::Sender<String>),
    AddressChanged(String),
    Connect,
    AmplitudeChanged(usize, String),
    FrequencyTextChanged(usize, String),
    SubmitFrequency(usize),
    PhaseChanged(usize, u16),
}

mod text_input_theme {
    use iced::{
        widget::text_input::{self, Appearance},
        Border, Color, Theme,
    };

    pub struct CustomBackgroundTextInput(pub Theme, pub Option<Color>);
    impl text_input::StyleSheet for CustomBackgroundTextInput {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            let palette = self.0.extended_palette();
            text_input::Appearance {
                background: self.1.unwrap_or(palette.background.base.color).into(),
                border: Border {
                    radius: 2.0.into(),
                    width: 1.0,
                    color: palette.background.strong.color,
                },
                icon_color: palette.background.weak.text,
            }
        }
        fn focused(&self, _style: &Self::Style) -> Appearance {
            let palette = self.0.extended_palette();
            text_input::Appearance {
                background: self.1.unwrap_or(palette.background.base.color).into(),
                border: Border {
                    radius: 2.0.into(),
                    width: 1.0,
                    color: palette.primary.strong.color,
                },
                icon_color: palette.background.weak.text,
            }
        }
        fn placeholder_color(&self, _style: &Self::Style) -> Color {
            let palette = self.0.extended_palette();
            palette.background.strong.color
        }
        fn value_color(&self, _style: &Self::Style) -> Color {
            let palette = self.0.extended_palette();
            palette.background.base.text
        }
        fn disabled_color(&self, _style: &Self::Style) -> Color {
            self.placeholder_color(_style)
        }
        fn selection_color(&self, _style: &Self::Style) -> Color {
            let palette = self.0.extended_palette();
            palette.primary.weak.color
        }
        fn disabled(&self, _style: &Self::Style) -> Appearance {
            let palette = self.0.extended_palette();
            text_input::Appearance {
                background: palette.background.weak.color.into(),
                border: Border {
                    radius: 2.0.into(),
                    width: 1.0,
                    color: palette.background.strong.color,
                },
                icon_color: palette.background.strong.color,
            }
        }
    }
}
