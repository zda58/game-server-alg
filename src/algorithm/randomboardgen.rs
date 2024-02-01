
use rand::Rng;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::data::{coordinates::{coord::{self, Coord}, coordstate::CoordState, owncoord::{generate_null_coord, OwnCoord}, statecoord::StateCoord}, ship::{shippiece::ShipPiece, shiptype::ShipType}};

pub struct RandomBoard {
    board: Vec<Vec<OwnCoord>>,
    ships: Vec<ShipPiece>
}

pub fn generate_random_board(spec: &HashMap<ShipType, u32>, height: usize, width: usize) -> (Vec<Vec<OwnCoord>>, Vec<Rc<RefCell<ShipPiece>>>) {
    let mut rand = rand::thread_rng();
    let mut board: Vec<Vec<OwnCoord>> = Vec::new();
    let mut validBoard = false;
    let mut ships: Vec<Rc<RefCell<ShipPiece>>> = Vec::new();
    while !validBoard {
        ships = Vec::new();
        board = vec![vec![generate_null_coord(); height]; width];
        for y in 0..height {
            for x in 0..width {
                board[x][y].x = x as u32;
            }
        }
        let mut overlap = false;
        for item in spec {
            let clone = item.clone();
            let shiptype = item.0.clone();
            for i in 0..item.1.clone() as usize {
                let randCoords: Vec<StateCoord>;
                if rand.gen_bool(0.5) {
                    randCoords = generate_horizontal_coords(&mut board, shiptype.len());
                } else {
                    randCoords = generate_vertical_coords(&mut board, shiptype.len());
                }
                let ship: ShipPiece = ShipPiece {
                    shipType: shiptype,
                    coords: randCoords.clone(),
                    reportedHitCoords: Vec::new()
                };

                let ship_rc = Rc::new(RefCell::new(ship));

                for statecoord in randCoords {
                    let mut coord: &mut OwnCoord = &mut board[statecoord.x as usize][statecoord.y as usize];
                    if coord.is_empty() {
                        coord.ship = Some(ship_rc.clone());
                    } else {
                        overlap = true;
                    }
                }
                ships.push(ship_rc);
            }
        }
        if !overlap {
            validBoard = true;
        }
    }
    (board, ships)
}

fn generate_horizontal_coords(board: &mut Vec<Vec<OwnCoord>>, length: usize) -> Vec<StateCoord> {
    let mut rand: rand::prelude::ThreadRng = rand::thread_rng();
    let coordY = rand.gen_range(0..board[0].len());
    let leftCoordX = rand.gen_range(0..board.len() - length + 1);

    let mut vec: Vec<StateCoord> = Vec::new();
    for x in leftCoordX..leftCoordX + length {
        vec.push(StateCoord{x: x as u32, y: coordY as u32, state: CoordState::NORMAL});
    }
    vec
}

fn generate_vertical_coords(board: &mut Vec<Vec<OwnCoord>>, length: usize) -> Vec<StateCoord> {
    let mut rand: rand::prelude::ThreadRng = rand::thread_rng();
    let coordX = rand.gen_range(0..board.len());
    let topCoordY = rand.gen_range(0..board.len() - length - 1);
    
    let mut vec: Vec<StateCoord> = Vec::new();
    for y in topCoordY..topCoordY + length {
        vec.push(StateCoord{x: coordX as u32, y: y as u32, state: CoordState::NORMAL});
    }
    vec
}