#![allow(clippy::assigning_clones)]

use std::{any::TypeId, array, future::pending, sync::mpsc, thread::spawn};

use anyhow::{bail, Context};
use iced::{
    futures::{
        channel::mpsc::{self as mpsc_f},
        SinkExt,
    },
    subscription, theme,
    widget::{button, column, horizontal_space, row, slider, text, text_input},
    Alignment, Application, Color, Command, Renderer, Settings, Subscription, Theme,
};
use log::error;
use regex::Regex;
use spcm::{
    CardMode, ClockMode, DdsCommand, Device, M2Command, TriggerMask, FREQUENCY_STEP, PHASE_STEP,
};
use text_input_theme::CustomBackgroundTextInput;

fn main() -> anyhow::Result<()> {
    env_logger::builder().format_timestamp_nanos().init();
    App::run(Settings {
        ..Default::default()
    })?;
    Ok(())
}

struct App {
    // dds_connector: Option<mpsc::Sender<String>>,
    address: String,
    connection_status: ConnectionStatus,
    // connected: bool,
    channel_states: [ChannelState; 4],
    status_message: String,
}
enum ConnectionStatus {
    Disconnected,
    Connecting,
    // Connected(UnboundedSender<DdsWorkerMessage>),
    Connected(mpsc::Sender<DdsWorkerMessage>),
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
        let mut app = App {
            // dds_connector: None,
            address: "/dev/spcm0".to_owned(),
            // connected: false,
            connection_status: ConnectionStatus::Disconnected,
            channel_states: array::from_fn(|_| ChannelState {
                amplitude: "100".into(),
                frequency_text: "1 000 000.0".into(),
                frequency_editing: false,
                frequency: 0, // Will be updated by "submit_frequency"
                phase: 0,
            }),
            status_message: "".to_owned(),
        };
        for i in 0..4 {
            app.update_frequency_text(i)
                .expect("The frequency text written above must be valid.");
        }
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Spectrum DDS".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // Message::Nop => {}
            // Message::EstablishSubscriptionConnection(sender) => self.dds_connector = Some(sender),
            Message::AddressChanged(x) => self.address = x,
            Message::Connect => {
                self.connection_status = ConnectionStatus::Connecting;
                self.status_message = "Connecting...".to_owned();
                // let Some(dds_connector) = &mut self.dds_connector else {
                //     self.report_error("Waiting for the DDS worker to start...".to_owned());
                //     return Command::none();
                // };
                // return Command::perform(dds_connector.send(self.address.clone()), |_| {
                //     Message::Nop
                // });
            }
            Message::ConnectionEstablished(tx) => {
                self.connection_status = ConnectionStatus::Connected(tx);
                self.status_message = "Connected.".to_owned();
                let ConnectionStatus::Connected(tx) = &self.connection_status else {
                    unreachable!();
                };
                if let Err(e) = (|| {
                    for (i, channel) in self.channel_states.iter().enumerate() {
                        tx.send(DdsWorkerMessage::SetFrequency(i, channel.frequency))?;
                        tx.send(DdsWorkerMessage::SetPhase(i, channel.phase))?;
                    }
                    anyhow::Ok(())
                })() {
                    self.report_error(format!("{e:#}"))
                }
            }
            Message::ConnectionLost(error) => {
                self.connection_status = ConnectionStatus::Disconnected;
                if let Some(error) = error {
                    self.status_message = error;
                }
            }
            Message::AmplitudeChanged(i, x) => self.channel_states[i].amplitude = x,
            Message::FrequencyTextChanged(i, x) => {
                let c = &mut self.channel_states[i];
                c.frequency_text = x;
                c.frequency_editing = true;
            }
            Message::SubmitFrequency(i) => {
                if let Err(e) = self.submit_frequency(i) {
                    self.report_error(format!("{e:#}"))
                }
            }
            Message::PhaseChanged(i, x) => {
                if let Err(e) = self.submit_phase(i, x) {
                    self.report_error(format!("{e:#}"))
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Message, Theme, Renderer> {
        type E<'a> = iced::Element<'a, Message, Theme, Renderer>;
        let address_row = {
            let address_input = {
                let mut t = text_input("", &self.address);
                if !self.connected() {
                    t = t
                        .on_input(Message::AddressChanged)
                        .on_submit(Message::Connect);
                }
                t
            };
            let connect_button =
                button("Connect").on_press_maybe((!self.connected()).then_some(Message::Connect));
            row![address_input, connect_button]
        };
        let mut column = column![address_row];
        for (i, channel) in self.channel_states.iter().enumerate() {
            let label = text(&format!("Channel {i}"));
            let amplitude_label = text("Amplitude:");
            let amplitude_input = {
                let mut t = text_input("80-2500", &channel.amplitude);
                if !self.connected() {
                    t = t.on_input(move |x| Message::AmplitudeChanged(i, x));
                }
                t
            };
            let frequency_label = text("Frequency:");
            let frequency_input = {
                let mut t =
                    text_input("", &channel.frequency_text).on_submit(Message::SubmitFrequency(i));
                if self.connected() {
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
            let error = text(&self.status_message);
            column = column.push(error);
        }
        column.into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let connecting = !matches!(self.connection_status, ConnectionStatus::Disconnected);
        let connection = connecting.then(|| {
            struct Connection;
            let address = self.address.clone();
            subscription::channel(TypeId::of::<Connection>(), 100, |mut tx| async move {
                spawn(move || {
                    let res = pollster::block_on(dds_worker(tx.clone(), address));
                    let error = res
                        .err()
                        .map(|e| format!("{:#}", e.context("Error occurred on DDS worker")));
                    if let Some(e) = &error {
                        error!("{e}");
                    }
                    if let Err(e) = pollster::block_on(tx.send(Message::ConnectionLost(error))) {
                        error!("Failed to send connection lost notification: {e:#}");
                    }
                });
                pending().await
            })
        });
        Subscription::batch(connection)
        // iced::subscription::channel((), 100, |mut tx| async move {
        //     let (tx_app, mut rx) = mpsc::channel(100);
        //     if let Err(e) = tx
        //         .send(Message::EstablishSubscriptionConnection(tx_app))
        //         .await
        //     {
        //         error!("Failed to establish subscription connection");
        //         pending().await
        //     }
        //     while let Some(address) = rx.next().await {
        //         let tx = tx.clone();
        //         spawn(move || pollster::block_on(dds_worker(tx, address)));
        //     }
        //     error!("End of stream");
        //     pending().await
        // })
    }
}

impl App {
    fn report_error(&mut self, message: String) {
        self.status_message = message
    }

    #[allow(clippy::inconsistent_digit_grouping)]
    fn update_frequency_text(&mut self, index: usize) -> anyhow::Result<u32> {
        let c = &mut self.channel_states[index];
        let frequency = c.parse_frequency()?;
        c.frequency = frequency;
        c.frequency_text = {
            let val = (frequency as f64 * FREQUENCY_STEP * 10.).round() as u64;
            let g = val / 1_000_000_000_0 % 1000;
            let m = val / 1_000_000_0 % 1000;
            let k = val / 1_000_0 % 1000;
            let o = val / 10 % 1000;
            let f = val % 10;
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
        Ok(frequency)
    }

    fn submit_frequency(&mut self, index: usize) -> anyhow::Result<()> {
        let frequency = self.update_frequency_text(index)?;
        // Send changes to DDS
        let ConnectionStatus::Connected(tx) = &self.connection_status else {
            bail!("Unreachable: connection status is not `Connected`!")
        };
        tx.send(DdsWorkerMessage::SetFrequency(index, frequency))?;
        Ok(())
    }

    fn submit_phase(&mut self, index: usize, phase: u16) -> anyhow::Result<()> {
        // TODO: phase slider should be "disabled"
        if let ConnectionStatus::Connected(tx) = &self.connection_status {
            self.channel_states[index].phase = phase;
            tx.send(DdsWorkerMessage::SetPhase(index, phase))?;
        };
        Ok(())
    }

    fn connected(&self) -> bool {
        matches!(self.connection_status, ConnectionStatus::Connected(_))
    }
}

#[derive(Debug)]
enum DdsWorkerMessage {
    SetFrequency(usize, u32),
    SetPhase(usize, u16),
}

async fn dds_worker(mut tx: mpsc_f::Sender<Message>, address: String) -> anyhow::Result<()> {
    let mut device = open_and_start_dds(&address)?;

    let (main_tx, rx) = mpsc::channel();
    tx.send(Message::ConnectionEstablished(main_tx)).await?;

    for message in rx.iter() {
        match message {
            DdsWorkerMessage::SetFrequency(index, frequency) => {
                let index = core_index(index);
                log::info!("Set frequency: {index} {frequency:08x};");
                device.dds_core_mut(index).set_frequency_exact(frequency)?;
            }
            DdsWorkerMessage::SetPhase(index, phase) => {
                let index = core_index(index);
                log::info!("Set phase: {index} {phase:04x};");
                device.dds_core_mut(index).set_phase_exact(phase)?;
            }
        }
        device.execute_dds_command(DdsCommand::ExecuteAtTrigger)?;
        device.execute_dds_command(DdsCommand::WriteToCard)?;
        device.execute_command(M2Command::CardForceTrigger)?;
    }

    device.execute_command(M2Command::CardStop)?;
    Ok(())
}

fn open_and_start_dds(address: &str) -> anyhow::Result<Device> {
    let mut device = Device::open(address)?;
    device.enable_channels(0b1111)?;
    device.set_card_mode(CardMode::StdDds)?;
    device.set_trigger_or_mask(TriggerMask::empty())?;
    device.set_clock_mode(ClockMode::InternalPll)?;
    device.enable_clock_out(true)?;
    for i in 0..4 {
        // TODO respect amplitude
        device.set_channel_amplitude(i, 500)?;
        device.enable_channel_out(i, true)?;
    }
    device.execute_command(M2Command::CardWriteSetup)?;
    device.execute_dds_command(DdsCommand::Reset)?;
    device.execute_command(M2Command::CardStart)?;
    for i in 0..4 {
        let mut core = device.dds_core_mut(core_index(i));
        core.set_amplitude(1.)?;
        core.set_phase(0.)?;
        core.set_frequency(1e6)?;
    }
    device.execute_dds_command(DdsCommand::ExecuteAtTrigger)?;
    device.execute_dds_command(DdsCommand::WriteToCard)?;
    device.execute_command(M2Command::CardForceTrigger)?;
    Ok(device)
}

fn core_index(index: usize) -> usize {
    assert!(index < 4);
    if index == 0 {
        0
    } else {
        19 + index
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
            let f: f64 = captures
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
        // u32 range \subset i64 range.
        // First convert to i64, then to u32, to check out-of-range
        ((value / FREQUENCY_STEP).round() as i64)
            .try_into()
            .context("Value out of range")
    }
}

#[derive(Clone, Debug)]
enum Message {
    // Nop,
    // EstablishSubscriptionConnection(mpsc::Sender<String>),
    AddressChanged(String),

    Connect,
    // ConnectionEstablished(UnboundedSender<DdsWorkerMessage>),
    ConnectionEstablished(mpsc::Sender<DdsWorkerMessage>),
    ConnectionLost(Option<String>),
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
