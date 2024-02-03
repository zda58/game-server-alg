use std::rc::Rc;

use crate::data::coordinates::{coord::Coord, heatmapcoord::HeatmapCoord};

pub struct HorizontalIterator {
    coord: Rc<HeatmapCoord>,
    board: Rc<Vec<Vec<HeatmapCoord>>>,
    priorityCoords: Vec<HeatmapCoord>,
    remainingCoords: Vec<HeatmapCoord>,
    shotCoords: Vec<Rc<HeatmapCoord>>,
    missedCoords: Vec<HeatmapCoord>,
    hitCoords: Vec<HeatmapCoord>,
    justShotCoords: Vec<HeatmapCoord>
}

impl HorizontalIterator {
    pub fn new() {

    }

    pub fn update_hits(&self) {
    }

    pub fn get_priority_shots(&self) -> Vec<Coord> {
        Vec::new()
    }
}
