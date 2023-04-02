use rand::Rng;
use std::fmt::{Display, Formatter};

pub enum SocketState {
    On,
    Off,
}

impl Display for SocketState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            SocketState::On => write!(f, "On"),
            SocketState::Off => write!(f, "Off"),
        }
    }
}

pub struct Socket {
    description: String,
    state: SocketState,
    current_power_consumption: f64,
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

impl Socket {
    pub fn new() -> Self {
        let description = String::from("new socket");
        Self {
            description,
            state: SocketState::Off,
            current_power_consumption: 0.0,
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, new_description: &str) {
        self.description = String::from(new_description);
    }

    pub fn get_state(&self) -> u8 {
        match self.state {
            SocketState::On => 1,
            SocketState::Off => 0,
        }
    }

    pub fn switch_on(&mut self) {
        let mut rng = rand::thread_rng();
        self.state = SocketState::On;
        self.current_power_consumption = rng.gen_range(1.0..30.0);
    }

    pub fn switch_off(&mut self) {
        self.state = SocketState::Off;
        self.current_power_consumption = 0.0;
    }

    pub fn get_current_power_consumption(&self) -> f64 {
        self.current_power_consumption
    }
}
