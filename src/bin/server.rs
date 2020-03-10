extern crate pretty_env_logger;
#[macro_use] extern crate log;
use tokio::net::{TcpListener};
use tokio::prelude::*;

pub type MyError = Box<dyn std::error::Error + Send + Sync>;


#[tokio::main]
async fn main() -> Result<(), MyError> {
    pretty_env_logger::init();
    let mut listener = TcpListener::bind("127.0.0.1:4567").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
		info!("We have a connection from: {}", socket.peer_addr().unwrap());

        tokio::spawn(async move {
            let mut cmd_id = [0; 1]; // First byte is a command id

            loop {
                let n = match socket.peek(&mut cmd_id).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                // Just turn a u8 into a OpCode
                let cmd = rotaro_node::parser::parse_operation(&cmd_id);
                info!("cmd: {:?}", cmd);
                // Match on whatever command we get (kind of pointless, could maybe just use the id here)
                match cmd {
                    rotaro_node::parser::OpCode::Handshake => {
                        // here we should read N bytes from the socket and parse that into a struct with bincode
                        // The code below is just to see if I can just output the byte count from a empty struct programmatically
                        let handshake = rotaro_node::parser::Handshake{id: 101, mac: *b"aaaabbbbcccc", bounce_count: 0, hub_time: 1337, node_time: 4848};
                        info!("here is a empty handshake: {:?}", handshake);
                    },
                    _ => {
                        info!("I don't know this command");
                    }
                }
                // Write the data back (remnants of the echo server I haven't removed)
                if let Err(e) = socket.write_all(&cmd_id[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
