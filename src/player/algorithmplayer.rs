use crate::data::coordinates::statecoord::StateCoord;
use crate::algorithm::algorithmmodel::AlgorithmModel;
use crate::data::ship::shippiece::ShipPiece;
pub struct AlgorithmPlayer {
    name: String,
    model: AlgorithmModel,
    ownBoard: Vec<Vec<StateCoord>>,
    otherBoard: Vec<Vec<StateCoord>>,
    ships: Vec<ShipPiece>
}

impl AlgorithmPlayer {
    pub fn name(&self) -> String {
        self.name
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


pub fn generate_algorithm_player() -> Option<AlgorithmPlayer> {
    None
}