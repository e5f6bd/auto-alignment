use sdl2::{event::Event, pixels::Color, rect::Rect};

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init()?;

    let w = 960;
    let h = 540;

    let video = sdl.video()?;
    let window = video.window("PAM controller", w, h).build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;

    let joystick = sdl.joystick()?;
    let _joystick = joystick.open(0)?;
    let mut joystick_position = [0; 4];

    let mut event = sdl.event_pump()?;

    'outer_loop: loop {
        for event in event.poll_iter() {
            match event {
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    joystick_position[axis_idx as usize] = dbg!(value) as i32;
                }
                Event::Quit { .. } => break 'outer_loop,
                _ => (),
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for i in 0..2 {
            let scale = 256;
            let x = w as i32 * (2 * i + 1) / 4 + joystick_position[2 * i as usize] / scale;
            let y = h as i32 / 2 + joystick_position[2 * i as usize + 1] / scale;
            let s = 20;
            canvas.set_draw_color(Color::RED);
            canvas.draw_rect(Rect::new(x - s / 2, y - s / 2, s as u32, s as u32))?;
        }

        canvas.present();
    }

    Ok(())
}
