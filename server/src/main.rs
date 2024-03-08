use game::init_game;
use local_ip_address::local_ip;
use std::net::{TcpListener, TcpStream};

use serverinfo::{self, data::gamesetup::GameSetup};
mod data;
mod game;
mod gamedata;
mod validation;

fn main() {
    let streams: (TcpStream, TcpStream) = init_connections();
    let setup = GameSetup::new(15, 15, 3, 3, 3, 3);
    init_game(streams.0, streams.1, setup);
}

fn init_connections() -> (TcpStream, TcpStream) {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let local_ip = local_ip().unwrap();
    println!("Listening on address: {}:{}", local_ip, addr.port());

    let first_stream = listener.incoming().find_map(|stream| stream.ok()).unwrap();
    let second_stream = listener.incoming().find_map(|stream| stream.ok()).unwrap();

    println!("Both acquired!");
    (first_stream, second_stream)
}
