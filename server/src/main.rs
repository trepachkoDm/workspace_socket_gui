use crate::socket_server::SocketServer;
use server::Socket;

mod socket_server;

fn main() -> std::io::Result<()> {
    let mut socket = Socket::new();
    socket.set_description("Socket with TCP server");

    let listener = SocketServer::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        SocketServer::handle_client(stream?, &mut socket)?;
    }
    Ok(())
}
