use game::init_game;
use local_ip_address::local_ip;
use core::time;
use std::{cmp, io, net::{TcpListener, TcpStream}, thread};

use serverinfo::{self, data::gamesetup::GameSetup};
mod data;
mod game;
mod gamedata;
mod validation;
mod view;

fn main() {
    let height = get_value(6, 50, "height of the board (between 6 and 50)");
    let width = get_value(6, 50, "width of the board (between 6 and 50)");
    let ships: (i32, i32, i32, i32) = get_ship_counts(cmp::max(height, width)); 
    let listener = init_port();
    loop {
        let streams: (TcpStream, TcpStream) = init_connections(&listener);
        let setup = GameSetup::new(height, width, ships.0, ships.1, ships.2, ships.3);
        println!("Game start!");
        init_game(streams.0, streams.1, setup);
        thread::sleep(time::Duration::from_millis(10000));
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

fn get_value(min: i32, max: i32, dim: &str) -> i32  {
    println!("Enter the desired {}", dim);
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }
        match input.trim().parse::<i32>() {
            Ok(count) => {
                if count >= min && count <= max {
                    return count;
                } else {
                    println!("Invalid range");
                    continue;
                }
            },
            Err(_) => {
                println!("Failed to parse count");
                continue;
            }
        }
    }
}

fn get_ship_counts(max: i32) -> (i32, i32, i32, i32)  {
    loop {
        let submarines: i32 = get_value(0, 50, "submarine count");
        let destroyers: i32 = get_value(0, 50, "destroyer count");
        let battleships: i32 = get_value(0, 50, "battleship count");
        let carriers: i32 = get_value(0, 50, "carrier count");
    
        if submarines + destroyers + battleships + carriers > max {
            println!("Ship count cannot exceed minimum dimension {}!", max);
            continue;
        } else {
            return (submarines, destroyers, battleships, carriers);
        }
    }
}