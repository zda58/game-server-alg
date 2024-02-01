use std::rc::Rc;

use crate::data::coordinates::heatmapcoord::HeatmapCoord;

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

