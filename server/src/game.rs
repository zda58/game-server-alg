use std::collections::HashSet;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;

use serverinfo::json::coord::Coord;
use serde::{de::DeserializeOwned, Serialize};
use serverinfo::json::gamestate::CurrentGameState;

use crate::{
    data::ship::Ship,
    gamedata::GameData,
    serverinfo::json::{
        gamesetup::GameSetup,
        report::Report,
        shipinfo::{ShipCoord, ShipInfo},
        shots::{ShotRequest, Shots},
    },
};
use serverinfo::json::gamestate::CurrentGameState::{Draw, Loss, Ongoing, Win};

pub fn init_game(p1stream: TcpStream, p2stream: TcpStream) {
    let mut p1reader = BufReader::new(p1stream.try_clone().unwrap());
    let mut p2reader = BufReader::new(p2stream.try_clone().unwrap());

    let setup = GameSetup::new(15, 15, 3, 3, 3, 3);

    report_data_to_client::<GameSetup>(&p1stream, &setup);
    let p1info: ShipInfo = get_data_from_client::<ShipInfo>(&mut p1reader).unwrap();
    validate_setup_info(&p1info, &setup);

    report_data_to_client::<GameSetup>(&p2stream, &setup);
    let p2info: ShipInfo = get_data_from_client::<ShipInfo>(&mut p2reader).unwrap();
    validate_setup_info(&p1info, &setup);

    let mut game = GameData::new(&setup, &p1info, &p2info);
    let mut p1state: CurrentGameState = Ongoing;
    let mut p2state: CurrentGameState = Ongoing;

    loop {
        let mut p1shots: Option<Vec<Coord>> = None;
        let mut p2shots: Option<Vec<Coord>> = None;
        let p1shotcount = get_shot_counts(&game.p1ships);
        let p2shotcount = get_shot_counts(&game.p2ships);
        if p1shotcount == 0 {
            if p2shotcount == 0 {
                p1state = Draw;
                p2state = Draw;
                break;
            } else {
                p1state = Loss;
                p2state = Win;
                break;
            }
        } else {
            if p2shotcount == 0 {
                p1state = Win;
                p2state = Loss;
                break;
            }
        }
        report_data_to_client::<CurrentGameState>(&p1stream, &p1state);
        report_data_to_client::<CurrentGameState>(&p2stream, &p2state);
        
        let shot_request = ShotRequest { shots: p1shotcount };
        report_data_to_client::<ShotRequest>(&p1stream, &shot_request);
        p1shots = Some(get_data_from_client::<Shots>(&mut p1reader).unwrap().shots);
        validate_shot_info(p1shots.as_ref().unwrap(), &shot_request, &setup);
        
        
        let shot_request = ShotRequest { shots: p2shotcount };
        report_data_to_client::<ShotRequest>(&p2stream, &shot_request);
        p2shots = Some(get_data_from_client::<Shots>(&mut p2reader).unwrap().shots);
        validate_shot_info(p2shots.as_ref().unwrap(), &shot_request, &setup);
        
        let mut p1_damaged_coords: Vec<Coord> = Vec::new();
        let mut p2_damaged_coords: Vec<Coord> = Vec::new();
        for ship in &mut game.p1ships {
            for coord in &p2shots.clone().unwrap() {
                ship.shoot_at(&Coord {
                    x: coord.x,
                    y: coord.y,
                });
            }
        }
        for ship in &mut game.p2ships {
            for coord in &p1shots.clone().unwrap() {
                ship.shoot_at(&Coord {
                    x: coord.x,
                    y: coord.y,
                });
            }
        }

        for ship in &mut game.p2ships {
            p2_damaged_coords.append(&mut ship.get_hit_coords());
        }
        for ship in &mut game.p1ships {
            p1_damaged_coords.append(&mut ship.get_hit_coords());
        }

        let p1report = Report {
            shots_hit: p2_damaged_coords.clone(),
            coords_damaged: p1_damaged_coords.clone()
        };
        report_data_to_client(&p1stream, &p1report);
        let p2report = Report {
            shots_hit: p2_damaged_coords,
            coords_damaged: p1_damaged_coords
        };
        report_data_to_client(&p2stream, &p2report);
    }
    report_data_to_client::<CurrentGameState>(&p1stream, &p1state);
    report_data_to_client::<CurrentGameState>(&p2stream, &p2state);
}

fn validate_setup_info(ship_info: &ShipInfo, setup: &GameSetup) -> bool {
    let submarines = setup.submarines;
    let destroyers = setup.destroyers;
    let battleships = setup.battleships;
    let carriers = setup.carriers;

    if (submarines != ship_info.submarines.len() as i32
        || destroyers != ship_info.destroyers.len() as i32
        || battleships != ship_info.battleships.len() as i32
        || carriers != ship_info.carriers.len() as i32)
    {
        println!("1");
        return false;
    }
    let mut coords: HashSet<Coord> = HashSet::new();
    for submarine in &ship_info.submarines {
        if !validate_ship_coords(setup, 3, submarine, &mut coords) {
            println!("2");
            return false;
        }
    }
    for destroyer in &ship_info.destroyers {
        if !validate_ship_coords(setup, 4, destroyer, &mut coords) {
            println!("3");
            return false;
        }
    }
    for battleship in &ship_info.battleships {
        if !validate_ship_coords(setup, 5, battleship, &mut coords) {
            println!("4");
            return false;
        }
    }
    for carrier in &ship_info.carriers {
        if !validate_ship_coords(setup, 6, carrier, &mut coords) {
            println!("5");
            return false;
        }
    }
    return true;
}

fn validate_ship_coords(
    setup: &GameSetup,
    shiplen: i32,
    coord: &ShipCoord,
    coords: &mut HashSet<Coord>,
) -> bool {
    let height = setup.height;
    let width = setup.width;
    if coord.horizontal {
        if (coord.x < 0 || coord.x > width - shiplen) || (coord.y < 0 || coord.y > height) {
            println!("01");
            return false;
        }
        for i in 0..shiplen {
            if coords.contains(&Coord {
                x: coord.x + i,
                y: coord.y,
            }) {
                println!("02");
                return false;
            } else {
                coords.insert(Coord {
                    x: coord.x + i,
                    y: coord.y,
                });
            }
        }
    } else {
        if (coord.x < 0 || coord.x > width) || (coord.y < 0 || coord.y > height - shiplen) {
            println!("03");
            return false;
        }
        for i in 0..shiplen {
            if coords.contains(&Coord {
                x: coord.x,
                y: coord.y + i,
            }) {
                println!("04");
                return false;
            } else {
                coords.insert(Coord {
                    x: coord.x,
                    y: coord.y + i,
                });
            }
        }
    }
    return true;
}

fn validate_shot_info(shots: &Vec<Coord>, request: &ShotRequest, setup: &GameSetup) -> bool {
    if shots.len() != request.shots as usize {
        return false;
    }
    for shot in shots {
        if shot.x < 0 || shot.x >= setup.width {
            return false;
        }
        if shot.y < 0 || shot.y >= setup.height {
            return false;
        }
    }
    true
}

fn get_shot_counts(ships: &Vec<Ship>) -> i32 {
    let mut count = 0;
    for ship in ships {
        if !ship.is_destroyed() {
            count += 1;
        }
    }
    count
}

fn get_data_from_client<T: DeserializeOwned>(reader: &mut BufReader<TcpStream>) -> Result<T, io::Error> {
    loop {
        let mut buffer = String::new();
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("Server closed");
                exit(0);
            }
            Ok(_) => {
                match serde_json::from_str::<T>(&buffer) {
                    Ok(report) => return Ok(report),
                    Err(e) => return Err(e.into())
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                return Err(e)
            }
        }
    }
}

fn report_data_to_client<T: Serialize>(mut stream: &TcpStream, data: &T) {
    let data = serde_json::to_string(data).unwrap();
    let write_data = format!("{}\n", data);
    stream.write_all(write_data.as_bytes());
    stream.flush();
}