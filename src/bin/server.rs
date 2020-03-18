use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub type MyError = Box<dyn std::error::Error + Send + Sync>;

fn handle_client(mut stream: TcpStream) {
	let mut cmd_id = [0; 1]; // First byte is a command id

	loop {
		let n = match stream.peek(&mut cmd_id) {
			// socket closed
			Ok(n) if n == 0 => return,
			Ok(n) => n,
			Err(e) => {
				error!("failed to read from socket; err = {:?}", e);
				return;
			}
		};
		// Just turn a u8 into a OpCode
		let cmd = rotaro_node::parser::parse_operation(&cmd_id);
		// Match on whatever command we get (kind of pointless, could maybe just use the id here)
		match cmd {
			rotaro_node::parser::OpCode::Handshake => {
				let cmd: rotaro_node::parser::Handshake  = bincode::deserialize_from(&stream).unwrap();
				info!("cmd is: {:?}", cmd);
				let data = bincode::serialize(&rotaro_node::parser::Handshake{id: 101, mac: *b"aaaabbbbcccc", bounce_count: 0, hub_time: 1337, node_time: 4848}).unwrap();
				stream.write(&data).unwrap();
			},
			_ => {
				info!("I don't know this command");
				}
			}
		}
}

fn main() {
    pretty_env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:4567").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 4567");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}

