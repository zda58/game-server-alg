use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use game::init_game;
use local_ip_address::local_ip;
use serde_json::{Deserializer, Serializer};
use serde_json::Result;


mod json;
mod gamestate;
mod data;
mod game;

fn main() {
    let mut streams: (TcpStream, TcpStream) = init_connections();
    init_game(streams.0, streams.1);
}

fn init_connections() -> (TcpStream, TcpStream) {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let local_ip = local_ip().unwrap();
    println!("Listening on address: {}:{}", local_ip, addr.port());
    
    let mut first_stream_option: Option<TcpStream> = None;
    let mut second_stream_option: Option<TcpStream> = None;
    for (mut idx, stream) in listener.incoming().enumerate() {
        let curstream = match stream {
            Ok(stream) => {
                if idx == 0 {
                    println!("First stream acquired");
                    first_stream_option = Some(stream);
                } else if idx == 1 {
                    println!("Second stream acquired");
                    second_stream_option = Some(stream);
                    break;
                }
            },
            _ => {
                idx -= 1;
            }
        };
    }
    println!("Both acquired!");
    (first_stream_option.unwrap(), second_stream_option.unwrap())
}
/*
    let player1: Player = Player {
        is_turn: false
    };
    let player2: Player = Player {
        is_turn: false
    };
    let mut p1ref = Arc::new(Mutex::new(player1));
    let mut p2ref = Arc::new(Mutex::new(player2));
 */