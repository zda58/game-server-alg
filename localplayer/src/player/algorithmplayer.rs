use std::collections::HashMap;

use std::{cell::RefCell, rc::Rc};


use crate::algorithm::randomboardgen;
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::owncoord::OwnCoord;
use crate::data::coordinates::statecoord::{CoordState, StateCoord};
use crate::algorithm::algorithmmodel::{AlgorithmModel};
use crate::data::ship::shippiece::{ShipPiece, ShipType};

pub struct AlgorithmPlayer {
    pub name: String,
    pub model: AlgorithmModel,
    pub own_board: Vec<Vec<OwnCoord>>,
    pub other_board: Vec<Vec<StateCoord>>,
    pub ships: Vec<Rc<RefCell<ShipPiece>>>
}

impl AlgorithmPlayer {
    pub fn new(name: String, spec: &HashMap<ShipType, u32>, width: usize, height: usize,) -> Self {
        let model = AlgorithmModel::new(spec, height, width);
        let boardships = randomboardgen::generate_board(spec, height, width);
        let mut other_board = vec![vec![StateCoord{x: 0, y: 0, state: CoordState::Normal}; width]; height];
        for x in 0..width {
            for y in 0..height {
                other_board[y][x].x = x as u32;
                other_board[y][x].y = y as u32;
            }
        }
        Self {
            name: name,
            model: model,
            own_board: boardships.0,
            other_board: other_board,
            ships: boardships.1
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn take_shots(&mut self) -> Vec<Coord> {
        let shots = self.model.take_shots(Self::get_ship_count(self));
        for shot in shots.iter() {
            self.other_board[shot.y as usize][shot.x as usize].shoot_at();
        }
        shots
    }

    pub fn report_damage(&mut self, shots: Vec<Coord>) -> Vec<Coord> {
        for coord in shots.iter() {
            self.own_board[coord.y as usize][coord.x as usize].get_shot();
        }
        let mut reported_hit_coords: Vec<Coord> = Vec::new(); 
        for shiprc in self.ships.iter() {
            reported_hit_coords.extend(shiprc.borrow_mut().report_coords());
        }
        reported_hit_coords
    }

    pub fn record_successful_hits(&mut self, hits: Vec<Coord>) {
        for coord in hits.iter() {
            let x = coord.x;
            let y = coord.y;
            self.other_board[y as usize][x as usize].hit_ship();
        }
        self.model.record_successful_hits(hits);
    }

    pub fn get_ship_count(&self) -> u32 {
        let count = self.ships.iter()
        .filter(|rc| rc.borrow().is_destroyed() == false)
        .count() as u32;
        count
    }

    pub fn draw_own_board(&self) {
        println!();
        println!("{} board", self.name);
        for y in 0..self.own_board.len() {
            for x in 0..self.own_board[0].len() {
                print!(" {} ", self.own_board[y][x].symbol());
            }
            println!();
        }
    }

        
}