use std::rc::Rc;

use crate::data::{coordinates::heatmapcoord::HeatmapCoord, ship::shippiece::ShipPiece};

use super::horizontaliterator::HorizontalIterator;

pub struct AlgorithmModel {
    ships: Rc<Vec<ShipPiece>>,
    otherboardHeatmap: Vec<Vec<HeatmapCoord>>,
    horizontalIterators: Vec<Vec<HorizontalIterator>>
}