use rand::Rng;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::data::{coordinates::{coord::{self, Coord}, owncoord::{generate_null_coord, OwnCoord}, statecoord::StateCoord}, ship::shippiece::{ShipPiece, ShipType}};

pub fn generate_board(spec: &HashMap<ShipType, u32>, height: usize, width: usize) -> (Vec<Vec<OwnCoord>>, Vec<Rc<RefCell<ShipPiece>>>) {
    let mut rand = rand::thread_rng();
    let mut board: Vec<Vec<OwnCoord>> = Vec::new();
    let mut valid_board = false;
    let mut ships: Vec<Rc<RefCell<ShipPiece>>> = Vec::new();
    while !valid_board {
        ships = Vec::new();
        board.clear();
        board = vec![vec![generate_null_coord(); height]; width];
        for y in 0..height {
            for x in 0..width {
                board[y][x].x = x as u32;
                board[y][x].y = y as u32;
            }
        }
        let mut overlap = false;
        for item in spec {
            let clone = item.clone();
            let shiptype = item.0.clone();
            for i in 0..item.1.clone() as usize {
                let rand_coords: Vec<Coord>;
                if rand.gen_bool(0.5) {
                    rand_coords = generate_horizontal_coords(&mut board, shiptype.len());
                } else {
                    rand_coords = generate_vertical_coords(&mut board, shiptype.len());
                }
                let ship: ShipPiece = ShipPiece {
                    ship_type: shiptype,
                    coords: rand_coords.clone(),
                    destroyed_coords: Vec::new(),
                    reported_hit_coords: Vec::new()
                };

                let ship_rc = Rc::new(RefCell::new(ship));

                for statecoord in rand_coords {
                    let mut coord: &mut OwnCoord = &mut board[statecoord.y as usize][statecoord.x as usize];
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
    (board, ships)
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