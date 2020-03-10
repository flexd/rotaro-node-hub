use std::io::{self, Read, Write};
use std::net::TcpStream;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    let mut stream = TcpStream::connect("127.0.0.1:4567").expect("Can't connect");

    info!("Connected to the server");

    let data = bincode::serialize(&rotaro_node::parser::Handshake{id: 101, mac: *b"aaaabbbbcccc", bounce_count: 0, hub_time: 1337, node_time: 4848}).unwrap();
    stream.write(&data).unwrap();
}
