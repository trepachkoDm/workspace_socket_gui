use iced::alignment::Alignment;
use iced::keyboard::KeyCode::Escape;
use iced::widget::{column, container, row, text, toggler};
use iced::{
    executor, keyboard, window, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use iced_native::Event;
use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;

pub fn main() -> iced::Result {
    WebSocket::run(Settings {
        window: window::Settings {
            size: (500, 500),
            resizable: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct WebSocket {
    state: bool,
    power: f64,
    status: String,
    should_exit: bool,
}

#[derive(Debug)]
pub enum Message {
    EventOccurred(Event),
    Synced(std::io::Result<(u8, f64)>),
    SwitchedOn(std::io::Result<()>),
    Toggle,
}

impl Application for WebSocket {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (WebSocket, Command<Self::Message>) {
        (
            WebSocket {
                state: false,
                power: 0.0,
                status: "Not connected".to_string(),
                should_exit: false,
            },
            Command::perform(get_state(), Message::Synced),
        )
    }

    fn title(&self) -> String {
        String::from("Hello! I am WebSocket")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Synced(state) => {
                if let Ok((state, pwr)) = state {
                    self.state = state == 1u8;
                    self.power = pwr;
                    self.status = "Synced".to_string()
                } else {
                    self.status = "Not connected".to_string()
                }
            }
            Message::Toggle => {
                if self.state {
                    self.state = false;
                    return Command::perform(switch_off(), Message::SwitchedOn);
                }
                self.state = true;
                return Command::perform(switch_on(), Message::SwitchedOn);
            }

            Message::SwitchedOn(result) => {
                if result.is_ok() {
                    return Command::perform(get_state(), Message::Synced);
                }
            }

            Message::EventOccurred(event) => {
                if let Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) = event {
                    if key_code == Escape {
                        self.should_exit = true;
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let title = text("Smart Socket Control").size(20);
        let power = text(format!("Power consumption: {:.2} A", self.power)).size(10);
        let status = text(format!("Status: {}", self.status)).size(10);

        let toggler = toggler(String::from("Toggle me!"), self.state, |_| Message::Toggle)
            .width(Length::Shrink)
            .spacing(10);

        let content = column![row![column![toggler, title, power, status,].spacing(20)]
            .spacing(10)
            .height(500)
            .align_items(Alignment::Center),]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
async fn get_state() -> std::io::Result<(u8, f64)> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all("cmd0".as_bytes())?;
    let mut buf: [u8; 12] = [0; 12];
    stream.read_exact(&mut buf)?;
    let msg = buf.as_slice();
    if &msg[0..3] == b"rst" {
        let state = msg[3];
        let pwr_buf: [u8; 8] = msg[4..].try_into().unwrap();
        let pwr = f64::from_be_bytes(pwr_buf);
        Ok((state, pwr))
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Corrupted message"))
    }
}

async fn switch_on() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all("cmd1".as_bytes())?;
    Ok(())
}

async fn switch_off() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all("cmd2".as_bytes())?;
    Ok(())
}
