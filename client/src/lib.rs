use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::TcpStream;

pub struct SocketClient {}

impl SocketClient {
    pub fn get_state() -> Result<(u8, f64)> {
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

    pub fn switch_on() -> Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        stream.write_all("cmd1".as_bytes())?;
        Ok(())
    }

    pub fn switch_off() -> Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        stream.write_all("cmd2".as_bytes())?;
        Ok(())
    }
}

#[repr(i32)]
pub enum SockError {
    NoError = 0,
    SwitchOnFailed,
    SwitchOffFailed,
    GetStatusFailed,
}

#[derive(Default)]
#[repr(C)]
pub struct SocketState {
    state: u32,
    power: f64,
    error: i32,
}

#[no_mangle]
pub extern "C" fn sync() -> SocketState {
    match SocketClient::get_state() {
        Ok((state, power)) => SocketState {
            state: state as u32,
            power,
            error: SockError::NoError as i32,
        },
        Err(_) => SocketState {
            error: SockError::GetStatusFailed as i32,
            ..Default::default()
        },
    }
}

#[no_mangle]
pub extern "C" fn switch_on() -> SockError {
    match SocketClient::switch_on() {
        Ok(_) => SockError::NoError,
        Err(_) => SockError::SwitchOnFailed,
    }
}

#[no_mangle]
pub extern "C" fn switch_off() -> SockError {
    match SocketClient::switch_off() {
        Ok(_) => SockError::NoError,
        Err(_) => SockError::SwitchOffFailed,
    }
}
