use crate::data::ship::Ship;
use serde::Serialize;
use serverinfo::data::coord::Coord;
use serverinfo::{
    self,
    data::{gamesetup::GameSetup, shipinfo::ShipInfo},
};

pub struct GameData {
    pub p1_ships: Vec<Ship>,
    pub p2_ships: Vec<Ship>,
}

impl GameData {
    pub fn new(setup: &GameSetup, p1info: &ShipInfo, p2info: &ShipInfo) -> Self {
        let mut p1ships: Vec<Ship> = Vec::new();
        Self::insert_ships(&mut p1ships, p1info);
        let mut p2ships: Vec<Ship> = Vec::new();
        Self::insert_ships(&mut p2ships, p2info);
        Self {
            p1_ships: p1ships,
            p2_ships: p2ships,
        }
    }

    fn insert_ships(ships: &mut Vec<Ship>, info: &ShipInfo) {
        for submarine in &info.submarines {
            let mut coords: Vec<Coord> = Vec::with_capacity(3);
            for i in 0..3 {
                if submarine.horizontal {
                    coords.push(Coord {
                        x: submarine.x + i,
                        y: submarine.y,
                    });
                } else {
                    coords.push(Coord {
                        x: submarine.x,
                        y: submarine.y + i,
                    });
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
        for destroyer in &info.destroyers {
            let mut coords: Vec<Coord> = Vec::with_capacity(4);
            for i in 0..4 {
                if destroyer.horizontal {
                    coords.push(Coord {
                        x: destroyer.x + i,
                        y: destroyer.y,
                    });
                } else {
                    coords.push(Coord {
                        x: destroyer.x,
                        y: destroyer.y + i,
                    });
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
        for battleship in &info.battleships {
            let mut coords: Vec<Coord> = Vec::with_capacity(5);
            for i in 0..5 {
                if battleship.horizontal {
                    coords.push(Coord {
                        x: battleship.x + i,
                        y: battleship.y,
                    });
                } else {
                    coords.push(Coord {
                        x: battleship.x,
                        y: battleship.y + i,
                    });
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
        for carrier in &info.carriers {
            let mut coords: Vec<Coord> = Vec::with_capacity(6);
            for i in 0..6 {
                if carrier.horizontal {
                    coords.push(Coord {
                        x: carrier.x + i,
                        y: carrier.y,
                    });
                } else {
                    coords.push(Coord {
                        x: carrier.x,
                        y: carrier.y + i,
                    });
                }
            }
            let ship = Ship::new(coords);
            ships.push(ship);
        }
    }
    pub fn empty() -> Self {
        Self {
            p1_ships: Vec::new(),
            p2_ships: Vec::new(),
        }
    }
}

#[derive(Serialize)]
pub struct GameResult {
    pub result: String,
}
