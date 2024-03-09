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
    println!("Enter the desired height of the board (between 6 and 50):");
    let height = get_dimension(6, 50);
    println!("Enter the desired width of the board (between 6 and 50):");
    let width = get_dimension(6, 50);
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

fn get_dimension(min: i32, max: i32) -> i32  {
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
    println!("Input ship counts like following: \"{{submarine count}} {{destroyer count}} {{battleship count}} {{carrier count}}\"");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }
        let mut nums = input.trim().split_whitespace();

        let submarines: i32 = match nums.next() {
            Some(num) => {
                match num.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to parse ships!");
                        continue;
                    },
                }
            },
            None => {
                println!("Failed to parse ships!");
                continue;
            }
        };
        let destroyers: i32 = match nums.next() {
            Some(num) => {
                match num.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to parse ships!");
                        continue;
                    },
                }
            },
            None => {
                println!("Failed to parse ships!");
                continue;
            }
        };
        let battleships: i32 = match nums.next() {
            Some(num) => {
                match num.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to parse ships!");
                        continue;
                    },
                }
            },
            None => {
                println!("Failed to parse ships!");
                continue;
            }
        };
        let carriers: i32 = match nums.next() {
            Some(num) => {
                match num.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to parse ships!");
                        continue;
                    },
                }
            },
            None => {
                println!("Failed to parse ships!");
                continue;
            }
        };
    
        if submarines + destroyers + battleships + carriers > max {
            println!("Ship count cannot exceed minimum dimension!");
            continue;
        } else {
            return (submarines, destroyers, battleships, carriers);
        }
    }
}