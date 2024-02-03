use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::data::{coordinates::{coord::Coord, heatmapcoord::HeatmapCoord}, ship::{shippiece::ShipPiece, shiptype::ShipType}};

use super::{horizontaliterator::HorizontalIterator, verticaliterator::VerticalIterator};

pub struct AlgorithmModel {
    possible_other_ships: Vec<ShipType>,
    other_board_heat_map: Vec<Vec<Rc<RefCell<HeatmapCoord>>>>,
    horizontal_iterators: Vec<HorizontalIterator>,
    vertical_iterators: Vec<VerticalIterator>,
    priority_coords: Vec<Coord>,
    remaining_coords: Vec<Rc<RefCell<HeatmapCoord>>>,
    shot_coords: Vec<Coord>,
    missed_coords: Vec<Coord>,
    hit_coords: Vec<Coord>,
    just_shot_coords: Vec<Coord>
}

impl AlgorithmModel {
    pub fn new(spec: &HashMap<ShipType, u32>, height: usize, width: usize) -> AlgorithmModel {
        let mut possible_other_ships: Vec<ShipType> = Vec::new();
        for (ship_type, count) in spec {
            for i in 0..count.clone() {
                possible_other_ships.push(ship_type.clone());
            }
        }
        let other_board_heat_map: Vec<Vec<Rc<RefCell<HeatmapCoord>>>> = vec![vec![Rc::new(RefCell::new(HeatmapCoord{x: 0, y: 0, heat: 0})); width]; height];
        let horizontal_iterators: Vec<HorizontalIterator> = Vec::new();
        let vertical_iterators: Vec<VerticalIterator> = Vec::new();
        let priority_coords: Vec<Coord> = Vec::new();
        let mut remaining_coords: Vec<Rc<RefCell<HeatmapCoord>>> = Vec::with_capacity(height * width);
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                remaining_coords[idx] = Rc::clone(&other_board_heat_map[y][x]);
            }
        }
        let shot_coords: Vec<Coord> = Vec::new();
        let missed_coords: Vec<Coord> = Vec::new();
        let hit_coords: Vec<Coord> = Vec::new();
        let just_shot_coords: Vec<Coord> = Vec::new();

        AlgorithmModel{
            possible_other_ships: possible_other_ships,
            other_board_heat_map: other_board_heat_map,
            horizontal_iterators: horizontal_iterators,
            vertical_iterators: vertical_iterators,
            priority_coords: priority_coords,
            remaining_coords: remaining_coords,
            shot_coords: shot_coords,
            missed_coords: missed_coords,
            hit_coords: hit_coords,
            just_shot_coords: just_shot_coords 
        }
    }

    fn update_heat_map(&mut self) {
        self.reset_heat_map();
        let mut list: Vec<Coord> = Vec::new();
        for ship_type in self.possible_other_ships.iter() {
            for y in 0..=self.other_board_heat_map.len() - ship_type.len() {
                for x in 0..self.other_board_heat_map[0].len() {
                    for i in 0..ship_type.len() {
                        list.push(Coord{x: x as u32, y: (y + i) as u32});
                    }
                }
            }
        }
        self.update_valid_position_coords(list);

        let mut list: Vec<Coord> = Vec::new();
        for ship_type in self.possible_other_ships.iter() {
            for y in 0..self.other_board_heat_map.len() {
                for x in 0..=self.other_board_heat_map[0].len() - ship_type.len() {
                    for i in 0..ship_type.len() {
                        list.push(Coord{x: (x + i) as u32, y: y as u32});
                    }
                }
            }
        }
        self.update_valid_position_coords(list);
        
        for coord in self.priority_coords.iter() {
            self.other_board_heat_map[coord.y as usize][coord.x as usize].borrow_mut().heat += 5000;
        }
        
    }

    fn update_valid_position_coords(&mut self, coords: Vec<Coord>) {
        for coord in coords.iter() {
            if self.missed_coords.contains(&coord) {
                return;
            }
        }
        for coord in coords.iter() {
            self.other_board_heat_map[coord.y as usize][coord.x as usize].borrow_mut().heat += 1;
        }
    }

    fn reset_heat_map(&mut self) {
        self.other_board_heat_map.iter_mut()
            .map(|vec| vec.iter_mut()
            .map(|coord| coord.borrow_mut().heat = 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    }

    pub fn record_successful_hits(&mut self, hits: Vec<Coord>) {
        for coord in hits.iter() {
            self.hit_coords.push(coord.clone());
        }
        for coord in self.shot_coords.iter() {
            if !self.hit_coords.contains(coord) && !self.missed_coords.contains(coord) {
                self.missed_coords.push(coord.clone());
            }
        }
        self.create_new_iterators(hits);
        self.priority_coords.clear();
        for iterator in self.vertical_iterators.iter() {
            iterator.update_hits();
            for coord in iterator.get_priority_shots() {
                if self.priority_coords.contains(&coord) {
                    self.priority_coords.push(coord);
                }
            }
        }
        for iterator in self.horizontal_iterators.iter() {
            iterator.update_hits();
            for coord in iterator.get_priority_shots() {
                if self.priority_coords.contains(&coord) {
                    self.priority_coords.push(coord);
                }
            }
        }
    }

    fn create_new_iterators(&mut self, shots_that_hit_opponent_ships: Vec<Coord>) {
        for coord in shots_that_hit_opponent_ships {
        }
    }

    pub fn take_shots(&mut self, mut num_shots: u32) -> Vec<Coord> {
        let mut shots: Vec<Coord> = Vec::new();
        if self.remaining_coords.len() < num_shots as usize {
            num_shots = self.remaining_coords.len() as u32
        }

        for _ in 0..num_shots {
            self.update_heat_map();
            self.remaining_coords
            .sort_by(|a, b| a.borrow_mut().heat.clone().cmp(&b.borrow_mut().heat));
            let coord = Coord{x: self.remaining_coords[0].borrow().x, y: self.remaining_coords[0].borrow().x};
            self.shot_coords.push(coord.clone());
            self.just_shot_coords.push(coord.clone());
            shots.push(coord.clone());
            self.remaining_coords.remove(0);
        }
        self.just_shot_coords.clear();
        self.update_heat_map();
        shots
    }
}
