use std::rc::Rc;

use crate::data::coordinates::heatmapcoord::HeatmapCoord;

struct VerticalIterator {
    coord: Rc<HeatmapCoord>,
    board: Rc<Vec<Vec<HeatmapCoord>>>,
    shotCoords: Vec<Rc<HeatmapCoord>>,
    //hitCoords: Vec<&HeatmapCoord>,
    //coords: Vec<&HeatmapCoord>,
    //top: Option<&HeatmapCoord>,
    //bottom: Option<&HeatmapCoord>
}


impl VerticalIterator {
    //pub fn get_priority_shots(coordsShot: Vec<&HeatmapCoord>) -> Vec<&HeatmapCoord> {

    //}
}