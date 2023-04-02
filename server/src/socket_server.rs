// use socket::crypt::Crypt;
use server::Socket;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct SocketServer {}

impl SocketServer {
    pub fn bind<Addrs>(addrs: Addrs) -> io::Result<TcpListener>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(tcp)
    }

    pub fn handle_client(mut stream: TcpStream, socket: &mut Socket) -> io::Result<()> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        match buf.as_slice() {
            b"cmd0" => {
                println!("Socket status requested");
                let state = socket.get_state();
                let pwr = socket.get_current_power_consumption();
                println!(
                    "State: {}, pwr: {:.2} A",
                    if state == 1 { "On" } else { "Off" },
                    pwr
                );
                let prefix = b"rst".as_slice();
                let pwr_bytes = pwr.to_be_bytes();
                let pwr_bytes_slice = pwr_bytes.as_slice();
                let state_array = [state];
                let state_slice = state_array.as_slice();
                let response = [prefix, state_slice, pwr_bytes_slice].concat();
                stream.write_all(&response)?;
            }
            b"cmd1" => {
                println!("Socket switch on invoked");
                socket.switch_on();
            }
            b"cmd2" => {
                println!("Socket switch off invoked");
                socket.switch_off();
            }
            _ => println!("received: bad command"),
        }
        Ok(())
    }
}
