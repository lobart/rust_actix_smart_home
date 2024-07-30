use iced::alignment::{self, Alignment};
use iced::widget::{button, column, row, text, text_input};
use iced::{Element, Length};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub fn main() -> iced::Result {
    iced::program("Smart House", SmartDevice::update, SmartDevice::view)
        .window_size((300.0, 400.0))
        .run()
}

struct SmartDevice {
    _id: String,
    input_url: String,
    device: Device,
    connected: bool,
    state: SmartDeviceState,
}

impl Default for SmartDevice {
    fn default() -> Self {
        SmartDevice {
            _id: String::from("0"),
            input_url: String::from("http://127.0.0.1:8080"),
            device: Device {
                id: String::from(""),
                name: String::from(""),
                type_: String::from(""),
                address: None,
                state: true,
                variable: 0,
                room: String::from(""),
            },
            connected: false,
            state: SmartDeviceState::Idle,
        }
    }
}
#[derive(Debug, Clone)]
pub enum SmartDeviceState {
    Idle,
    Connected,
}

impl Default for SmartDeviceState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum SmartDeviceMessage {
    Connected(bool),
    Connect,
    InputChanged(String),
    TurnLamp,
    Delete,
}

impl SmartDevice {
    pub fn text_state(s: bool) -> &'static str {
        if s {
            "Turn Off"
        } else {
            "Turn on"
        }
    }

    fn _new(url: String, device: Device, connected: bool) -> Self {
        SmartDevice {
            _id: Uuid::new_v4().to_string(),
            input_url: url,
            device,
            connected,
            state: SmartDeviceState::default(),
        }
    }

    fn update(&mut self, message: SmartDeviceMessage) {
        match message {
            SmartDeviceMessage::Connect => {
                let list_of_devices =
                    reqwest::blocking::get(format!("{}/devices-list", &self.input_url))
                        .unwrap()
                        .text()
                        .unwrap();
                if list_of_devices.contains("No devices") {
                    todo!()
                }
                let devices: Vec<&str> = serde_json::from_str(&list_of_devices).unwrap();
                let device_id: &str = devices[0];
                println!(
                    "{}/device/{}",
                    &self.input_url,
                    device_id.trim_start_matches('"')
                );
                let device = reqwest::blocking::get(format!(
                    "{0}/device/{1}",
                    &self.input_url,
                    device_id.trim_start_matches('"')
                ))
                .unwrap()
                .text()
                .unwrap();
                let d: Device = serde_json::from_str(&device).unwrap();
                self.device = d;
                self.connected = true;
                self.state = SmartDeviceState::Connected;
            }
            SmartDeviceMessage::Connected(connected) => {
                self.connected = connected;
            }
            SmartDeviceMessage::TurnLamp => {
                reqwest::blocking::get(format!(
                    "{0}/device/{1}/state",
                    &self.input_url, &self.device.id
                ))
                .unwrap()
                .text()
                .unwrap();
                let device = reqwest::blocking::get(format!(
                    "{0}/device/{1}",
                    &self.input_url, &self.device.id
                ))
                .unwrap()
                .text()
                .unwrap();
                let d: Device = serde_json::from_str(&device).unwrap();
                self.device = d;
            }
            SmartDeviceMessage::Delete => {}
            SmartDeviceMessage::InputChanged(value) => {
                self.input_url = value;
            }
        }
    }

    fn view(&self) -> Element<SmartDeviceMessage> {
        match &self.state {
            SmartDeviceState::Idle => {
                let url_text = text_input("Input url of server", &self.input_url)
                    .on_input(SmartDeviceMessage::InputChanged)
                    .width(Length::Fill)
                    .size(17)
                    .padding(10);

                row![
                    url_text,
                    button("Connect")
                        .on_press(SmartDeviceMessage::Connect)
                        .padding(10)
                        .style(button::text),
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            }
            SmartDeviceState::Connected => {
                let text_name = text(self.device.name.to_string())
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(20);
                let text_type = text(self.device.type_.to_string())
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(20);
                let text_status = text(format!("{}", &self.device.state))
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(20);
                let text_variable = text(format!("{}", &self.device.variable))
                    .vertical_alignment(alignment::Vertical::Center)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(20);
                let button_turn = button(Self::text_state(self.device.state))
                    .on_press(SmartDeviceMessage::TurnLamp);
                column![
                    text_name,
                    text_type,
                    row![text_status, button_turn],
                    text_variable
                ]
                .spacing(10)
                .align_items(Alignment::Center)
                .padding(10)
                .into()
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub address: Option<String>,
    pub state: bool,
    pub variable: i32,
    pub room: String,
}
