use std::collections::HashSet;

use serverinfo::data::{
    coord::Coord,
    gamesetup::GameSetup,
    shipinfo::{ShipCoord, ShipInfo},
    shots::ShotRequest,
};

use crate::data::ship::Ship;

pub fn validate_setup_info(ship_info: &ShipInfo, setup: &GameSetup) -> bool {
    let submarines = setup.submarines;
    let destroyers = setup.destroyers;
    let battleships = setup.battleships;
    let carriers = setup.carriers;

    if submarines != ship_info.submarines.len() as i32
        || destroyers != ship_info.destroyers.len() as i32
        || battleships != ship_info.battleships.len() as i32
        || carriers != ship_info.carriers.len() as i32
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

pub fn validate_ship_coords(
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

pub fn validate_shot_info(shots: &Vec<Coord>, request: &ShotRequest, setup: &GameSetup) -> bool {
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

pub fn get_shot_counts(ships: &Vec<Ship>) -> i32 {
    let mut count = 0;
    for ship in ships {
        if !ship.is_destroyed() {
            count += 1;
        }
    }
    count
}
