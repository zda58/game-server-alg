use game::init_game;
use local_ip_address::local_ip;
use core::time;
use std::{net::{TcpListener, TcpStream}, thread};

use serverinfo::{self, data::gamesetup::GameSetup};
mod data;
mod game;
mod gamedata;
mod validation;
mod view;

fn main() {
    let listener = init_port();
    loop {
        let streams: (TcpStream, TcpStream) = init_connections(&listener);
        let setup = GameSetup::new(15, 15, 3, 3, 3, 3);
        println!("Game start!");
        init_game(streams.0, streams.1, setup);
        thread::sleep(time::Duration::from_millis(2000));
    }
}

fn init_port() -> TcpListener {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let local_ip = local_ip().unwrap();
    println!("Listening on address: {}:{}", local_ip, addr.port());
    listener
}

fn init_connections(listener: &TcpListener) -> (TcpStream, TcpStream) {
    let first_stream = listener.incoming().find_map(|stream| stream.ok()).unwrap();
    println!("First stream acquired: {}", first_stream.peer_addr().unwrap());
    let second_stream = listener.incoming().find_map(|stream| stream.ok()).unwrap();
    println!("Second stream acquired: {}", second_stream.peer_addr().unwrap());
    (first_stream, second_stream)
}
