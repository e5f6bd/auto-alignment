// TODO: remove
#![allow(unused)]

use std::f64::consts::TAU;

use sdl2::{event::Event, pixels::Color};

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init()?;

    let w = 960;
    let h = 540;

    let video = sdl.video()?;
    let window = video.window("PAM controller", w, h).build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;

    let joystick = sdl.joystick()?;
    let _joystick = joystick.open(0)?;
    // let mut managers = vec![JoystickAxisManagerWithIndicator::default(); 2];

    let mut state = UiState::new();

    let mut event = sdl.event_pump()?;

    'outer_loop: loop {
        for event in event.poll_iter() {
            match event {
                Event::JoyButtonDown { button_idx, .. } => {
                    use JoyButton::*;
                    match button_idx {
                        0 => state.handle_button(A),
                        1 => state.handle_button(B),
                        11 => state.handle_button(Up),
                        12 => state.handle_button(Down),
                        13 => state.handle_button(Left),
                        14 => state.handle_button(Right),
                        _ => {}
                    }
                }
                // Sometimes, the + button is recognized as these events
                Event::JoyHatMotion {
                    hat_idx,
                    state: joy_state,
                    ..
                } => {
                    dbg!();
                    if hat_idx == 0 {
                        use JoyButton::*;
                        match joy_state {
                            sdl2::joystick::HatState::Up => state.handle_button(Up),
                            sdl2::joystick::HatState::Down => state.handle_button(Down),
                            sdl2::joystick::HatState::Left => state.handle_button(Left),
                            sdl2::joystick::HatState::Right => state.handle_button(Right),
                            _ => {}
                        }
                    }
                }
                Event::JoyAxisMotion {
                    axis_idx, value, ..
                } => {
                    // if let Some(manager) = managers.get_mut(axis_idx as usize / 2) {
                    //     manager.update(axis_idx as usize % 2, value);
                    // }
                }
                Event::Quit { .. } => break 'outer_loop,
                _ => (),
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // for (i, manager) in managers.iter().enumerate() {
        //     let x = w as i32 * (2 * i + 1) as i32 / 4;
        //     let y = h as i32 / 2;
        //     let theta = TAU * manager.indicator_position as f64 / 36.;
        //     let length = 200.;
        //     let dx = (theta.cos() * length) as i32;
        //     let dy = (-theta.sin() * length) as i32;
        //     canvas.set_draw_color(Color::RED);
        //     canvas.draw_line((x, y), (x + dx, y + dy))?;
        // }

        canvas.present();
    }

    Ok(())
}

struct UiState {
    axis_choices: Vec<Vec<AxisChoice>>,
    selections: [usize; 2],
    mode: Mode,
}
#[derive(Clone)]
struct AxisChoice(usize);
enum Mode {
    Selecting(bool), // choosing .0 as usize
    Operating,
}

#[derive(Debug)]
enum JoyButton {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
}

impl UiState {
    fn new() -> Self {
        let mut axis_choices = vec![Vec::<AxisChoice>::new(); 4];
        for i in 0..22 {
            axis_choices[i / 6].push(AxisChoice(i));
        }
        Self {
            axis_choices,
            selections: [0, 1],
            mode: Mode::Selecting(false),
        }
    }

    fn handle_button(&self, button: JoyButton) {
        dbg!(button);
    }
}

#[derive(Clone, Default)]
struct JoystickAxisManagerWithIndicator {
    indicator_position: i16,
    manager: JoystickAxisManager,
}
impl JoystickAxisManagerWithIndicator {
    fn update(&mut self, axis: usize, value: i16) -> i16 {
        let delta = self.manager.update(axis, value);
        self.indicator_position = (self.indicator_position + delta).rem_euclid(36);
        delta
    }
}

#[derive(Clone, Default)]
struct JoystickAxisManager {
    values: [i16; 2],
    last_position: Option<i16>,
}

impl JoystickAxisManager {
    fn update(&mut self, axis: usize, value: i16) -> i16 {
        self.values[axis] = value;

        let threashold_center = 25_000_f64;

        if self.values.iter().map(|&v| (v as f64).powi(2)).sum::<f64>() < threashold_center.powi(2)
        {
            self.last_position = None;
        } else {
            let modulus = 8;
            let next_position = (f64::atan2(self.values[0] as _, self.values[1] as _)
                / (TAU / modulus as f64)) as i16;
            if let Some(last_position) = self.last_position.replace(next_position) {
                let next_position = next_position
                    + if next_position < last_position {
                        modulus
                    } else {
                        0
                    };
                let next = if last_position + modulus / 2 < next_position {
                    next_position - modulus
                } else {
                    next_position
                };
                return next - last_position;
            }
        }

        0
    }
}
