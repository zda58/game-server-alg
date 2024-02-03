use std::rc::Rc;

use crate::data::coordinates::{coord::Coord, heatmapcoord::HeatmapCoord};

pub struct VerticalIterator {
    coord: Rc<HeatmapCoord>,
    board: Rc<Vec<Vec<HeatmapCoord>>>,
    shot_coords: Vec<Rc<HeatmapCoord>>,
    //hitCoords: Vec<&HeatmapCoord>,
    //coords: Vec<&HeatmapCoord>,
    //top: Option<&HeatmapCoord>,
    //bottom: Option<&HeatmapCoord>
}

impl VerticalIterator {
    pub fn new() {

    }

    pub fn update_hits(&self) {
    }

    pub fn get_priority_shots(&self) -> Vec<Coord> {
        Vec::new()
    }
}