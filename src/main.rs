use sdl2::{event::Event, pixels::Color};

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;
    let mut event = sdl.event_pump()?;

    let window = video
        .window("PAM controller", 960, 540)
        .allow_highdpi()
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;

    'outer_loop: loop {
        for event in event.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'outer_loop;
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
    }

    Ok(())
}
