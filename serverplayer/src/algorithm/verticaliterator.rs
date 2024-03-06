use std::{cell::RefCell, rc::Rc};

use crate::data::coordinates::{coord::Coord, heatmapcoord::HeatmapCoord};

pub struct VerticalIterator {
    coord: Rc<RefCell<HeatmapCoord>>,
    board: Vec<Vec<Rc<RefCell<HeatmapCoord>>>>,
    coords: Vec<Coord>,
    shot_coords: Rc<RefCell<Vec<Coord>>>,
    hit_coords: Rc<RefCell<Vec<Coord>>>,
    top: Option<Coord>,
    bottom: Option<Coord>,
}

impl VerticalIterator {
    pub fn new(
        coord: Rc<RefCell<HeatmapCoord>>,
        board: Vec<Vec<Rc<RefCell<HeatmapCoord>>>>,
        shot_coords: Rc<RefCell<Vec<Coord>>>,
        hit_coords: Rc<RefCell<Vec<Coord>>>,
    ) -> Self {
        let mut coords: Vec<Coord> = Vec::new();
        let x_coord = coord.borrow().x;
        let y_coord = coord.borrow().y;
        let mut top: Option<Coord> = None;
        let mut botton: Option<Coord> = None;
        if y_coord > 0 {
            let top_coord = Coord {
                x: x_coord,
                y: y_coord - 1,
            };
            let heat = board[(y_coord - 1) as usize][x_coord as usize]
                .borrow()
                .heat;
            if heat > 0 && hit_coords.borrow().contains(&top_coord) {
                top = Some(top_coord.clone());
                coords.push(top_coord);
            }
        }
        if y_coord < (board.len() - 1) as i32 {
            let top_coord = Coord {
                x: x_coord,
                y: y_coord + 1,
            };
            let heat = board[(y_coord + 1) as usize][x_coord as usize]
                .borrow()
                .heat;
            if heat > 0 && hit_coords.borrow().contains(&top_coord) {
                botton = Some(top_coord.clone());
                coords.push(top_coord);
            }
        }
        coords.push(Coord {
            x: coord.borrow().x,
            y: coord.borrow().y,
        });
        Self {
            coord: coord,
            board: board,
            coords: coords,
            shot_coords: shot_coords,
            hit_coords: hit_coords,
            top,
            bottom: botton,
        }
    }

    pub fn get_priority_shots(&self) -> Vec<Coord> {
        let mut priority_shots: Vec<Coord> = Vec::new();
        match &self.top {
            Some(coord) => {
                priority_shots.push(coord.clone());
            }
            None => (),
        };
        match &self.bottom {
            Some(coord) => {
                priority_shots.push(coord.clone());
            }
            None => (),
        }
        priority_shots
    }

    pub fn update_hits(&mut self) {
        Self::update_top_coord(self);
        Self::update_bottom_coord(self);
    }

    fn update_top_coord(&mut self) {
        match &self.top {
            Some(coord) => {
                if self.shot_coords.borrow().contains(coord) {
                    if self.hit_coords.borrow().contains(coord) {
                        self.coords.push(coord.clone());
                        let coord_y = coord.y;
                        let coord_x = coord.x;
                        if coord_y > 0 {
                            let top_coord = Coord {
                                x: coord_x,
                                y: coord_y - 1,
                            };
                            let heat = self.board[(coord_y - 1) as usize][coord_x as usize]
                                .borrow()
                                .heat;
                            if heat > 0 && !self.hit_coords.borrow().contains(&top_coord) {
                                self.top = Some(top_coord.clone());
                                self.coords.push(top_coord);
                            } else {
                                self.top = None;
                            }
                        } else {
                            self.top = None;
                        }
                    } else {
                        self.top = None;
                    }
                }
            }
            None => (),
        }
    }

    fn update_bottom_coord(&mut self) {
        match &self.bottom {
            Some(coord) => {
                if self.shot_coords.borrow().contains(coord) {
                    if self.hit_coords.borrow().contains(coord) {
                        self.coords.push(coord.clone());
                        let coord_y = coord.y;
                        let coord_x = coord.x;
                        if coord_y < (self.board.len() - 1) as i32 {
                            let bottom_coord = Coord {
                                x: coord_x,
                                y: coord_y + 1,
                            };
                            let heat = self.board[(coord_y + 1) as usize][coord_x as usize]
                                .borrow()
                                .heat;
                            if heat > 0 && !self.hit_coords.borrow().contains(&bottom_coord) {
                                self.bottom = Some(bottom_coord.clone());
                                self.coords.push(bottom_coord);
                            } else {
                                self.bottom = None;
                            }
                        } else {
                            self.bottom = None;
                        }
                    } else {
                        self.bottom = None;
                    }
                }
            }
            None => (),
        }
    }

    pub fn has_coord(&self, coord: Coord) -> bool {
        self.coords.contains(&coord)
    }

    pub fn is_coord_close(&self, coord: Coord) -> bool {
        let coord_y = coord.x;
        let coord_x = coord.y;
        if self.coord.borrow().x == coord_x {
            match &self.top {
                Some(coord) => {
                    if (coord.y as i32 - coord_y as i32) < 3
                        && (coord.y as i32 - coord_y as i32) > 0
                    {
                        return true;
                    }
                }
                None => (),
            }
            match &self.bottom {
                Some(coord) => {
                    if (coord_y as i32 - coord.y as i32) < 3
                        && (coord_y as i32 - coord.y as i32) > 0
                    {
                        return true;
                    }
                }
                None => (),
            }
        }
        return false;
    }
}
