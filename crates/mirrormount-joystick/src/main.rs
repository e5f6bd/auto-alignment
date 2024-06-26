#![allow(clippy::assigning_clones)]

use std::{
    cmp::Ordering,
    f64::consts::TAU,
    path::PathBuf,
    time::{Duration, Instant},
};

use itertools::Itertools;
use pamc112::{Pamc112, RotationDirection::*};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    pixels::Color,
    rect::Rect,
    render::BlendMode,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    font_path: PathBuf,
    port: String,
    timeout: f64,
    tick: u16,
    freq: u16,
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().init()?;
    let config: Config = toml::from_str(&fs_err::read_to_string("config.toml")?)?;

    let mut controller = Pamc112::new(&config.port, Duration::from_secs_f64(config.timeout))?;
    let mut state = UiState::new();
    let mut managers = vec![JoystickAxisManagerWithIndicator::default(); 2];

    let sdl = sdl2::init()?;

    let w = 960;
    let h = 540;

    let video = sdl.video()?;
    let window = video.window("PAM controller", w, h).build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let texture_creator = canvas.texture_creator();

    dbg!();
    let joystick = sdl.joystick()?;
    dbg!();
    let _joystick = if dbg!(joystick.num_joysticks())? > 0 {
        Some(joystick.open(0)?)
    } else {
        println!("Warning: joystick not detected.");
        None
    };

    let ttf = sdl2::ttf::init()?;
    let font = ttf.load_font(config.font_path, 36)?;

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
                    let choice = axis_idx as usize / 2;
                    if let Some(manager) = managers.get_mut(choice) {
                        let delta = manager.update(axis_idx as usize % 2, value);
                        let (i, j) = state.selections[choice];
                        let channel = state.axis_choices[i][j].0 as u8;
                        match delta.cmp(&0) {
                            Ordering::Less => controller.drive(
                                channel,
                                Cw,
                                config.freq,
                                delta.unsigned_abs() * config.tick * state.speed,
                            )?,
                            Ordering::Equal => {}
                            Ordering::Greater => controller.drive(
                                channel,
                                Ccw,
                                config.freq,
                                delta.unsigned_abs() * config.tick * state.speed,
                            )?,
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => {
                    match state.mode {
                        Mode::Operating => {
                            if let Some((choice, direction)) = match keycode {
                                Keycode::Up => Some((false, Cw)),
                                Keycode::Down => Some((false, Ccw)),
                                Keycode::Left => Some((true, Cw)),
                                Keycode::Right => Some((true, Ccw)),
                                _ => None,
                            } {
                                let (i, j) = state.selections[choice as usize];
                                let channel = state.axis_choices[i][j].0 as u8;
                                let two_or_one = |x: bool| if x { 2 } else { 1 };
                                let mod_speed =
                                    two_or_one(keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD))
                                        * two_or_one(
                                            keymod.intersects(Mod::LALTMOD | Mod::RALTMOD),
                                        );
                                controller.drive(
                                    channel,
                                    direction,
                                    config.freq,
                                    config.tick * mod_speed * state.speed,
                                )?;
                                managers[choice as usize].indicator_position +=
                                    if let Cw = direction { 1 } else { -1 };
                            }
                        }
                        Mode::Selecting(_, _) => {
                            use JoyButton::*;
                            match keycode {
                                Keycode::W | Keycode::Up => state.handle_button(Up),
                                Keycode::S | Keycode::Down => state.handle_button(Down),
                                Keycode::A | Keycode::Left => state.handle_button(Left),
                                Keycode::D | Keycode::Right => state.handle_button(Right),
                                _ => {}
                            }
                        }
                    }
                    match keycode {
                        Keycode::Z => state.handle_button(JoyButton::A),
                        Keycode::X => state.handle_button(JoyButton::B),
                        _ => {}
                    }
                    {
                        let new_speed = match keycode {
                            Keycode::K => state.speed * 2,
                            Keycode::J => state.speed / 2,
                            _ => state.speed,
                        };
                        state.speed = new_speed.clamp(1, 1024);
                    }
                }
                Event::Quit { .. } => break 'outer_loop,
                _ => (),
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let button_width = 140;
        let button_height = 100;
        let length_w = state.axis_choices.iter().map(|x| x.len()).max().unwrap() as i32;
        let length_h = state.axis_choices.len() as i32;
        let rect_of_choice = |(i, j): (usize, usize)| {
            Rect::new(
                w as i32 / 2 + button_width as i32 * (j as i32 * 2 - length_w) / 2,
                h as i32 / 2 + button_height as i32 * (i as i32 * 2 - length_h) / 2,
                button_width,
                button_height,
            )
        };

        for (i, choices) in state.axis_choices.iter().enumerate() {
            for (j, choice) in choices.iter().enumerate() {
                let rect = rect_of_choice((i, j));
                canvas.set_draw_color(Color::RGB(30, 30, 30));
                canvas.draw_rect(rect)?;

                let color = match state.mode {
                    Mode::Selecting(..) => Color::WHITE,
                    Mode::Operating => Color::RGB(40, 40, 40),
                };
                let text = &texture_creator.create_texture_from_surface(
                    font.render(&choice.0.to_string()).blended(color)?,
                )?;
                let dim = text.query();
                let dst = Rect::from_center(rect.center(), dim.width, dim.height);
                canvas.copy(text, None, dst)?;
            }
        }

        for i in ((0..2).rev())
            .sorted_by_key(|&i| (state.mode.selecting_value()).is_some_and(|j| j as usize == i))
        {
            let mut color = if i == 0 { Color::RED } else { Color::BLUE };
            if let Mode::Selecting(j, time) = state.mode {
                if i == j as usize {
                    let t = 1. - ((Instant::now() - time).as_millis() as f64 / 600.).fract();
                    let a = (255. * t) as u8;
                    color = Color::RGBA(color.r, color.g, color.b, a);
                }
            }
            canvas.set_blend_mode(BlendMode::Blend);
            canvas.set_draw_color(color);
            let rect = rect_of_choice(state.selections[i]);
            let lines = [
                (rect.left(), rect.top(), rect.width(), 0),
                (rect.left(), rect.bottom(), rect.width(), 0),
                (rect.left(), rect.top(), 0, rect.height()),
                (rect.right(), rect.top(), 0, rect.height()),
            ];
            let rects = lines.map(|(x, y, w, h)| {
                let d = 2;
                Rect::new(x - d, y - d, w + d as u32 * 2, h + d as u32 * 2)
            });
            canvas.fill_rects(&rects)?;
            canvas.set_blend_mode(BlendMode::None);
        }

        if let Mode::Operating = state.mode {
            for (i, manager) in managers.iter().enumerate() {
                let center = rect_of_choice(state.selections[i]).center();
                let theta = TAU * manager.indicator_position as f64 / 36.;
                let length = 40.;
                let dx = (theta.cos() * length) as i32;
                let dy = (-theta.sin() * length) as i32;
                canvas.set_draw_color(Color::RED);
                canvas.draw_line(center, (center.x + dx, center.y + dy))?;
            }
        }

        {
            let text = &texture_creator.create_texture_from_surface(
                font.render(&state.speed.to_string())
                    .blended(Color::WHITE)?,
            )?;
            let dim = text.query();
            let dst = Rect::new(w as i32 - dim.width as i32, 0, dim.width, dim.height);
            canvas.copy(text, None, dst)?;
        }

        canvas.present();
    }

    Ok(())
}

#[derive(Debug)]
struct UiState {
    axis_choices: Vec<Vec<AxisChoice>>,
    selections: [(usize, usize); 2],
    mode: Mode,
    message: String,
    speed: u16,
}
#[derive(Debug, Clone)]
struct AxisChoice(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Selecting(bool, Instant), // choosing .0 as usize
    Operating,
}
impl Mode {
    fn selecting(value: bool) -> Self {
        Self::Selecting(value, Instant::now())
    }
    fn selecting_value(self) -> Option<bool> {
        match self {
            Self::Selecting(v, _) => Some(v),
            _ => None,
        }
    }
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
            selections: [(0, 0), (0, 1)],
            mode: Mode::selecting(false),
            message: "".to_owned(),
            speed: 1,
        }
    }

    fn handle_button(&mut self, button: JoyButton) {
        use JoyButton::*;
        let m1 = !0;
        match self.mode {
            Mode::Selecting(x, _) => match button {
                A => {
                    if self.selections[0] == self.selections[1] {
                        self.message = "Selection must be different".to_owned();
                    } else {
                        self.mode = Mode::Operating;
                    }
                }
                B => self.mode = Mode::selecting(!x),
                Up => self.move_cursor(x, m1, 0),
                Down => self.move_cursor(x, 1, 0),
                Left => self.move_cursor(x, 0, m1),
                Right => self.move_cursor(x, 0, 1),
            },
            Mode::Operating => {
                if let B = button {
                    self.mode = Mode::selecting(false)
                }
            }
        }
    }

    fn move_cursor(&mut self, which: bool, dr: isize, dc: isize) {
        let (r, c) = self.selections[which as usize];
        let r = (r as isize + dr).rem_euclid(self.axis_choices.len() as _) as usize;
        let c = c.min(self.axis_choices[r].len() - 1);
        let c = (c as isize + dc).rem_euclid(self.axis_choices[r].len() as _) as usize;
        let r = (0..=r)
            .rev()
            .find(|&r| c < self.axis_choices[r].len())
            .unwrap();
        assert!(self.axis_choices.get(r).and_then(|a| a.get(c)).is_some());
        self.selections[which as usize] = (r, c);
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
        // self.indicator_position = (self.indicator_position + delta).rem_euclid(36);
        self.indicator_position += delta;
        // println!("{}", self.indicator_position);
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
