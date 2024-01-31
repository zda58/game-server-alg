use std::collections::HashMap;

use crate::algorithm::randomboardgen::generate_random_board;
use crate::data::coordinates::coordstate::CoordState;
use crate::data::coordinates::owncoord::OwnCoord;
use crate::data::coordinates::statecoord::StateCoord;
use crate::algorithm::algorithmmodel::AlgorithmModel;
use crate::data::ship::shippiece::ShipPiece;
use crate::data::ship::shiptype::ShipType;
use rand::Rng;
pub struct AlgorithmPlayer {
    pub name: String,
    //model: AlgorithmModel,
    pub ownBoard: Vec<Vec<OwnCoord>>,
    pub otherBoard: Vec<Vec<StateCoord>>,
    pub ships: Vec<ShipPiece>
}

impl AlgorithmPlayer {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn setup(&self) {

    }

    pub fn take_shots(&self) {

    }

    pub fn report_damage(&self) {

    }

    pub fn successfulHits(&self) {

    }

    pub fn endGame(&self) {

    }
}


pub fn generate_algorithm_player(name: String, spec: &HashMap<ShipType, u32>, width: usize, height: usize,) -> AlgorithmPlayer {
    let mut rand = rand::thread_rng();
    let boardships = generate_random_board(spec, height, width);
    let mut otherboard = vec![vec![StateCoord{x: 0, y: 0, state: CoordState::NORMAL}; height]; width];
    for x in 0..width {
        for y in 0..height {
            otherboard[x][y].x = x as u32;
            otherboard[x][y].y = y as u32;
        }
    }
    AlgorithmPlayer {
        name: name,
        ownBoard: boardships.0,
        otherBoard: otherboard,
        ships: boardships.1
    }
}