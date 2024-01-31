use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::data::ship::shippiece::ShipPiece;

#[derive(Clone)]
pub struct OwnCoord {
    pub x: u32,
    pub y: u32,
    pub ship: Option<Rc<RefCell<ShipPiece>>>,
    pub shot: bool
}

impl OwnCoord {
    pub fn get_shot(&mut self) {
        self.shot = true;
        match &self.ship {
            Some(ship) => ship.borrow_mut().get_shot(self.x, self.y),
            _ => ()
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.ship {
            Some(_) => false,
            _ => true,
        }
    }

    pub fn set_ship(&mut self, ship: Rc<RefCell<ShipPiece>>) {
        self.ship = Some(ship);
    }
}

pub fn generate_null_coord() -> OwnCoord {
    OwnCoord {
        x: 0,
        y: 0,
        ship: None,
        shot: false
    }
}