use std::collections::HashMap;

use std::{cell::RefCell, rc::Rc};


use crate::algorithm::randomboardgen::{self, RandomBoard};
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::coordstate::CoordState;
use crate::data::coordinates::owncoord::OwnCoord;
use crate::data::coordinates::statecoord::StateCoord;
use crate::algorithm::algorithmmodel::{AlgorithmModel};
use crate::data::ship::shippiece::ShipPiece;
use crate::data::ship::shiptype::ShipType;
use rand::Rng;
pub struct AlgorithmPlayer {
    pub name: String,
    pub model: AlgorithmModel,
    pub own_board: Vec<Vec<OwnCoord>>,
    pub other_board: Vec<Vec<StateCoord>>,
    pub ships: Vec<Rc<RefCell<ShipPiece>>>
}

impl AlgorithmPlayer {
    pub fn new(name: String, spec: &HashMap<ShipType, u32>, width: usize, height: usize,) -> AlgorithmPlayer {
        let model = AlgorithmModel::new(spec, height, width);
        let boardships = RandomBoard::new(spec, height, width);
        let mut other_board = vec![vec![StateCoord{x: 0, y: 0, state: CoordState::Normal}; width]; height];
        for x in 0..width {
            for y in 0..height {
                other_board[y][x].x = x as u32;
                other_board[y][x].y = y as u32;
            }
        }
        AlgorithmPlayer {
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

    pub fn take_shots(&self) -> Vec<Coord> {
        Vec::new()
    }

    pub fn report_damage(&self, shots: Vec<Coord>) -> Vec<Coord> {
        Vec::new()
    }

    pub fn record_successful_hits(&self, hits: Vec<Coord>) {

    }

    pub fn get_ship_count(&self) -> u32 {
        self.ships.iter()
        .filter(|rc| rc.borrow().is_destroyed())
        .count() as u32
    }
}