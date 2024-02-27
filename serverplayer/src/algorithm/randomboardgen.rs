use rand::Rng;
use shipjson::json::{gamesetup::GameSetup, shipinfo::{ShipCoord, ShipInfo}};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};
use crate::data::{coordinates::{coord::{self, Coord}, owncoord::{generate_null_coord, OwnCoord}, statecoord::StateCoord}, ship::shippiece::{ShipPiece, ShipType}};

pub fn generate_board(setup: &GameSetup) -> (Vec<Vec<OwnCoord>>, Vec<Rc<RefCell<ShipPiece>>>, ShipInfo) {
    let mut rand = rand::thread_rng();
    let mut board: Vec<Vec<OwnCoord>> = Vec::new();
    let mut valid_board = false;
    let mut ships: Vec<Rc<RefCell<ShipPiece>>> = Vec::new();
    let mut ships_json: ShipInfo = ShipInfo::new();
    while !valid_board {
        ships.clear();
        ships_json.clear();
        board.clear();
        board = vec![vec![generate_null_coord(); setup.height as usize]; setup.width as usize];
        for y in 0..setup.height as usize {
            for x in 0..setup.width as usize {
                board[y][x].x = x as u32;
                board[y][x].y = y as u32;
            }
        }
        let mut overlap = false;
        let mut spec: HashMap<ShipType, i32> = HashMap::new();
        spec.insert(ShipType::Submarine, setup.submarines);
        spec.insert(ShipType::Destroyer, setup.destroyers);
        spec.insert(ShipType::Battleship, setup.battleships);
        spec.insert(ShipType::Carrier, setup.carriers);
        for item in spec {
            let shiptype = item.0.clone();
            for _ in 0..item.1.clone() as usize {
                let rand_coords: Vec<Coord>;
                let mut horizontal = true;
                if rand.gen_bool(0.5) {
                    rand_coords = generate_horizontal_coords(&mut board, shiptype.len());
                } else {
                    rand_coords = generate_vertical_coords(&mut board, shiptype.len());
                    horizontal = false;
                }
                let ship: ShipPiece = ShipPiece {
                    ship_type: shiptype,
                    coords: rand_coords.clone(),
                    destroyed_coords: Vec::new(),
                    reported_hit_coords: Vec::new()
                };

                let central_coord = rand_coords[0].clone();

                let json_coord: ShipCoord = ShipCoord {
                    horizontal: horizontal,
                    x: central_coord.x as i32,
                    y: central_coord.y as i32
                };

                match shiptype {
                    ShipType::Submarine => ships_json.submarines.push(json_coord),
                    ShipType::Destroyer => ships_json.destroyers.push(json_coord),
                    ShipType::Battleship => ships_json.battleships.push(json_coord),
                    ShipType::Carrier => ships_json.carriers.push(json_coord)
                }

                let ship_rc = Rc::new(RefCell::new(ship));

                for statecoord in rand_coords {
                    let coord: &mut OwnCoord = &mut board[statecoord.y as usize][statecoord.x as usize];
                    if coord.is_empty() {
                        coord.ship = Some(Rc::clone(&ship_rc));
                    } else {
                        overlap = true;
                    }
                }
                ships.push(ship_rc);
            }
        }
        if !overlap {
            valid_board = true;
        }
    }
    (board, ships, ships_json)
}

fn generate_horizontal_coords(board: &mut Vec<Vec<OwnCoord>>, length: usize) -> Vec<Coord> {
    let mut rand: rand::prelude::ThreadRng = rand::thread_rng();
    let coord_y = rand.gen_range(0..board[0].len());
    let left_coord_x = rand.gen_range(0..board.len() - length + 1);

    let mut vec: Vec<Coord> = Vec::new();
    for x in left_coord_x..left_coord_x + length {
        vec.push(Coord{x: x as u32, y: coord_y as u32});
    }
    vec
}

fn generate_vertical_coords(board: &mut Vec<Vec<OwnCoord>>, length: usize) -> Vec<Coord> {
    let mut rand: rand::prelude::ThreadRng = rand::thread_rng();
    let coord_x = rand.gen_range(0..board.len());
    let top_coord_y = rand.gen_range(0..board.len() - length - 1);
    
    let mut vec: Vec<Coord> = Vec::new();
    for y in top_coord_y..top_coord_y + length {
        vec.push(Coord{x: coord_x as u32, y: y as u32});
    }
    vec
}