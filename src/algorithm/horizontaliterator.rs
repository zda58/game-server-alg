use std::{cell::RefCell, rc::Rc};

use crate::data::coordinates::{coord::Coord, heatmapcoord::HeatmapCoord};

pub struct HorizontalIterator {
    coord:  Rc<RefCell<HeatmapCoord>>,
    board: Vec<Vec<Rc<RefCell<HeatmapCoord>>>>,
    coords: Vec<Coord>,
    shot_coords: Rc<RefCell<Vec<Coord>>>,
    hit_coords: Rc<RefCell<Vec<Coord>>>,
    left: Option<Coord>,
    right: Option<Coord>
}

impl HorizontalIterator {
    pub fn new(coord: Rc<RefCell<HeatmapCoord>>, board: Vec<Vec<Rc<RefCell<HeatmapCoord>>>>, 
        shot_coords: Rc<RefCell<Vec<Coord>>>, hit_coords: Rc<RefCell<Vec<Coord>>>) -> HorizontalIterator {
            let mut coords: Vec<Coord> = Vec::new();
            let x_coord = coord.borrow().x;
            let y_coord = coord.borrow().y;
            let mut left: Option<Coord> = None;
            let mut right: Option<Coord> = None;
            if x_coord > 0 {
                let left_coord = Coord{x: x_coord - 1, y: y_coord};
                let heat = board[y_coord as usize][(x_coord - 1) as usize].borrow().heat;
                if heat > 0 && hit_coords.borrow().contains(&left_coord) {
                    left = Some(left_coord.clone());
                    coords.push(left_coord);
                }
            }
            if x_coord < (board[0].len() - 1) as u32 {
                let right_coord = Coord{x: x_coord + 1, y: y_coord};
                let heat = board[y_coord as usize][(x_coord + 1) as usize].borrow().heat;
                if heat > 0 && hit_coords.borrow().contains(&right_coord) {
                    right = Some(right_coord.clone());
                    coords.push(right_coord);
                }
            }
            coords.push(Coord{x: coord.borrow().x, y: coord.borrow().y});
            HorizontalIterator {
                coord: coord,
                board: board,
                coords: coords,
                shot_coords: shot_coords,
                hit_coords: hit_coords,
                left: left,
                right: right, 

            }
    }

    pub fn get_priority_shots(&self) -> Vec<Coord> {
        let mut priority_shots: Vec<Coord> = Vec::new();
        match &self.left {
            Some(coord) => {
               priority_shots.push(coord.clone()); 
            },
            None => (),
        };
        match &self.right {
            Some(coord) => {
               priority_shots.push(coord.clone()); 
            },
            None => (),
        }
        priority_shots
    }

    pub fn update_hits(&mut self) {
        Self::update_left_coord(self);
        Self::update_right_coord(self);
    }

    fn update_left_coord(&mut self) {
        match &self.left {
            Some(coord) => {
                if self.shot_coords.borrow().contains(coord) {
                    if self.hit_coords.borrow().contains(coord) {
                        self.coords.push(coord.clone());
                        let coord_y = coord.y;
                        let coord_x = coord.x;
                        if coord_x > 0 {
                            let left_coord = Coord{x: coord_x - 1, y: coord_y};
                            let heat = self.board[coord_y as usize][(coord_x - 1) as usize].borrow().heat;
                            if heat > 0 && !self.hit_coords.borrow().contains(&left_coord) {
                                self.left = Some(left_coord.clone());
                                self.coords.push(left_coord);
                            } else {
                                self.left = None;
                            }
                        } else {
                            self.left = None;
                        }
                    } else {
                        self.left = None;
                    }
                }
            },
            None => (),
        }
    }

    fn update_right_coord(&mut self) {
        match &self.right {
            Some(coord) => {
                if self.shot_coords.borrow().contains(coord) {
                    if self.hit_coords.borrow().contains(coord) {
                        self.coords.push(coord.clone());
                        let coord_y = coord.y;
                        let coord_x = coord.x;
                        if coord_x > 0 {
                            let right_coord = Coord{x: coord_x + 1, y: coord_y};
                            let heat = self.board[coord_y as usize][(coord_x + 1) as usize].borrow().heat;
                            if heat > 0 && !self.hit_coords.borrow().contains(&right_coord) {
                                self.right = Some(right_coord.clone());
                                self.coords.push(right_coord);
                            } else {
                                self.right = None;
                            }
                        } else {
                            self.right = None;
                        }
                    } else {
                        self.right = None;
                    }
                }
            },
            None => (),
        }
    }

    pub fn has_coord(&self, coord: Coord) -> bool {
        self.coords.contains(&coord)        
    }

    pub fn is_coord_close(&self, coord: Coord) -> bool {
        let coord_y = coord.x;
        let coord_x = coord.y;
        if self.coord.borrow().y == coord_y {
            match &self.left {
                Some(coord) => {
                    if coord.x - coord_x < 3 && coord.x - coord_x > 0 {
                        return true;
                    }
                },
                None => (),
            }
            match &self.right {
                Some(coord) => {
                    if coord_x - coord.x < 3 && coord_x - coord.x > 0 {
                        return true;
                    }
                },
                None => (),
            }
        }
        return false;
    }
}
