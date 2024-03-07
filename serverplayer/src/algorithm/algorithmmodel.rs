use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serverinfo::json::{coord::Coord, gamesetup::{self, GameSetup}};

use crate::data::{
    coordinates::{heatmapcoord::HeatmapCoord},
    ship::shippiece::ShipType,
};

use super::{horizontaliterator::HorizontalIterator, verticaliterator::VerticalIterator};

pub struct AlgorithmModel {
    possible_other_ships: Vec<ShipType>,
    other_board_heat_map: Vec<Vec<Rc<RefCell<HeatmapCoord>>>>,
    horizontal_iterators: Vec<HorizontalIterator>,
    vertical_iterators: Vec<VerticalIterator>,
    priority_coords: Vec<Coord>,
    remaining_coords: Vec<Rc<RefCell<HeatmapCoord>>>,
    shot_coords: Rc<RefCell<Vec<Coord>>>,
    missed_coords: Vec<Coord>,
    hit_coords: Rc<RefCell<Vec<Coord>>>,
    just_shot_coords: Vec<Coord>,
}

impl AlgorithmModel {
    pub fn new(setup: &GameSetup) -> Self {
        let mut possible_other_ships: Vec<ShipType> = Vec::new();
        for _ in 0..setup.submarines {
            possible_other_ships.push(ShipType::Submarine.clone());
        }
        for _ in 0..setup.destroyers {
            possible_other_ships.push(ShipType::Destroyer.clone());
        }
        for _ in 0..setup.battleships {
            possible_other_ships.push(ShipType::Battleship.clone());
        }
        for _ in 0..setup.carriers {
            possible_other_ships.push(ShipType::Carrier.clone());
        }
        let mut other_board_heat_map: Vec<Vec<Rc<RefCell<HeatmapCoord>>>> =
            Vec::with_capacity(setup.height as usize);
        for y in 0..setup.height as usize {
            other_board_heat_map.push(Vec::with_capacity(setup.width as usize));
            for x in 0..setup.width {
                other_board_heat_map[y].push(Rc::new(RefCell::new(HeatmapCoord {
                    x: x as i32,
                    y: y as i32,
                    heat: 0,
                })));
            }
        }
        let horizontal_iterators: Vec<HorizontalIterator> = Vec::new();
        let vertical_iterators: Vec<VerticalIterator> = Vec::new();
        let priority_coords: Vec<Coord> = Vec::new();
        let mut remaining_coords: Vec<Rc<RefCell<HeatmapCoord>>> =
            Vec::with_capacity(setup.height as usize * setup.width as usize);
        for y in 0..setup.height as usize {
            for x in 0..setup.width as usize {
                remaining_coords.push(Rc::clone(&other_board_heat_map[y][x]));
            }
        }
        let shot_coords: Rc<RefCell<Vec<Coord>>> = Rc::new(RefCell::new(Vec::new()));
        let missed_coords: Vec<Coord> = Vec::new();
        let hit_coords: Rc<RefCell<Vec<Coord>>> = Rc::new(RefCell::new(Vec::new()));
        let just_shot_coords: Vec<Coord> = Vec::new();

        Self {
            possible_other_ships: possible_other_ships,
            other_board_heat_map: other_board_heat_map,
            horizontal_iterators: horizontal_iterators,
            vertical_iterators: vertical_iterators,
            priority_coords: priority_coords,
            remaining_coords: remaining_coords,
            shot_coords: shot_coords,
            missed_coords: missed_coords,
            hit_coords: hit_coords,
            just_shot_coords: just_shot_coords,
        }
    }

    fn update_heat_map(&mut self) {
        self.reset_heat_map();
        let ships = self.possible_other_ships.clone();
        for ship_type in &ships {
            for y in 0..=self.other_board_heat_map.len() - ship_type.len() {
                for x in 0..self.other_board_heat_map[0].len() {
                    let mut list: Vec<Coord> = Vec::new();
                    for i in 0..ship_type.len() {
                        list.push(Coord {
                            x: x as i32,
                            y: (y + i) as i32,
                        });
                    }
                    self.update_valid_position_coords(list);
                }
            }
        }
        let ships = self.possible_other_ships.clone();
        for ship_type in &ships {
            for y in 0..self.other_board_heat_map.len() {
                for x in 0..=self.other_board_heat_map[0].len() - ship_type.len() {
                    let mut list: Vec<Coord> = Vec::new();
                    for i in 0..ship_type.len() {
                        list.push(Coord {
                            x: (x + i) as i32,
                            y: y as i32,
                        });
                    }
                    self.update_valid_position_coords(list);
                }
            }
        }

        for coord in self.priority_coords.iter() {
            self.other_board_heat_map[coord.y as usize][coord.x as usize]
                .borrow_mut()
                .heat += 5000;
        }
    }

    fn update_valid_position_coords(&mut self, coords: Vec<Coord>) {
        for coord in coords.iter() {
            if self.missed_coords.contains(&coord) || self.just_shot_coords.contains(&coord) {
                return;
            }
        }
        for coord in coords.iter() {
            self.other_board_heat_map[coord.y as usize][coord.x as usize]
                .borrow_mut()
                .heat += 1;
        }
    }

    fn reset_heat_map(&mut self) {
        for row in &self.other_board_heat_map {
            for coord in row {
                coord.borrow_mut().heat = 0;
            }
        }
    }

    pub fn record_successful_hits(&mut self, hits: Vec<Coord>) {
        for coord in hits.iter() {
            self.hit_coords.borrow_mut().push(coord.clone());
        }
        for coord in self.shot_coords.borrow().iter() {
            if !self.hit_coords.borrow_mut().contains(coord) && !self.missed_coords.contains(coord)
            {
                self.missed_coords.push(coord.clone());
            }
        }
        self.create_new_iterators(hits);
        self.priority_coords.clear();
        for iterator in self.vertical_iterators.iter_mut() {
            iterator.update_hits();
            for coord in iterator.get_priority_shots() {
                if self.priority_coords.contains(&coord) {
                    self.priority_coords.push(coord);
                }
            }
        }
        for iterator in self.horizontal_iterators.iter_mut() {
            iterator.update_hits();
            for coord in iterator.get_priority_shots() {
                if self.priority_coords.contains(&coord) {
                    self.priority_coords.push(coord);
                }
            }
        }
    }

    fn create_new_iterators(&mut self, shots_that_hit_opponent_ships: Vec<Coord>) {
        for coord in shots_that_hit_opponent_ships.iter() {
            let hit_coord = &self.other_board_heat_map[coord.x as usize][coord.y as usize];
            let mut already_iterator = false;
            let mut close_vertical = false;
            let mut close_horizontal = false;
            for iterator in self.vertical_iterators.iter_mut() {
                already_iterator = iterator.has_coord(Coord {
                    x: hit_coord.borrow().x,
                    y: hit_coord.borrow().y,
                }) || already_iterator;
                close_vertical = iterator.is_coord_close(Coord {
                    x: hit_coord.borrow().x,
                    y: hit_coord.borrow().y,
                }) || close_horizontal;
                iterator.update_hits();
            }
            for iterator in self.horizontal_iterators.iter_mut() {
                already_iterator = iterator.has_coord(Coord {
                    x: hit_coord.borrow().x,
                    y: hit_coord.borrow().y,
                }) || already_iterator;
                close_vertical = iterator.is_coord_close(Coord {
                    x: hit_coord.borrow().x,
                    y: hit_coord.borrow().y,
                }) || close_horizontal;
                iterator.update_hits();
            }
            if !already_iterator {
                if (close_vertical && close_horizontal) || (!close_vertical && !close_horizontal) {
                    let horizontal_iter = HorizontalIterator::new(
                        Rc::clone(&hit_coord),
                        self.other_board_heat_map.clone(),
                        Rc::clone(&self.shot_coords),
                        Rc::clone(&self.hit_coords),
                    );
                    let vertical_iter = VerticalIterator::new(
                        Rc::clone(&hit_coord),
                        self.other_board_heat_map.clone(),
                        Rc::clone(&self.shot_coords),
                        Rc::clone(&self.hit_coords),
                    );
                    self.horizontal_iterators.push(horizontal_iter);
                    self.vertical_iterators.push(vertical_iter);
                } else if close_vertical {
                    let vertical_iter = VerticalIterator::new(
                        Rc::clone(&hit_coord),
                        self.other_board_heat_map.clone(),
                        Rc::clone(&self.shot_coords),
                        Rc::clone(&self.hit_coords),
                    );
                    self.vertical_iterators.push(vertical_iter);
                } else {
                    let horizontal_iter = HorizontalIterator::new(
                        Rc::clone(&hit_coord),
                        self.other_board_heat_map.clone(),
                        Rc::clone(&self.shot_coords),
                        Rc::clone(&self.hit_coords),
                    );
                    self.horizontal_iterators.push(horizontal_iter);
                }
            }
        }
    }

    pub fn take_shots(&mut self, mut num_shots: u32) -> Vec<Coord> {
        let mut shots: Vec<Coord> = Vec::new();
        if self.remaining_coords.len() < num_shots as usize {
            num_shots = self.remaining_coords.len() as u32
        }

        for i in 0..num_shots {
            self.update_heat_map();
            self.remaining_coords
                .sort_by(|a, b| b.borrow().heat.clone().cmp(&a.borrow().heat));
            let zerocoord = &self.remaining_coords[0];
            let coord = Coord {
                x: zerocoord.borrow().x,
                y: zerocoord.borrow().y,
            };
            self.shot_coords.borrow_mut().push(coord.clone());
            self.just_shot_coords.push(coord.clone());
            shots.push(coord.clone());
            self.remaining_coords.remove(0);
        }
        self.just_shot_coords.clear();
        self.update_heat_map();
        for coord in shots.iter() {}
        shots
    }

    fn print_heat_map(&self) {
        for y in 0..self.other_board_heat_map.len() {
            for x in 0..self.other_board_heat_map[0].len() {
                let heat = self.other_board_heat_map[y][x].borrow().heat;
                print!(" {} ", heat);
            }
            println!();
        }
    }
}
