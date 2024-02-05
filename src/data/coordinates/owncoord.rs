use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::data::ship::shippiece::ShipPiece;

use super::coord::Coord;

#[derive(Clone)]
pub struct OwnCoord {
    pub x: u32,
    pub y: u32,
    pub ship: Option<Rc<RefCell<ShipPiece>>>,
    pub shot: bool
}

impl OwnCoord {
    pub fn get_shot(&mut self) {
        println!("ship shot!!");
        self.shot = true;
        match &self.ship {
            Some(ship) => 
                ship.borrow_mut().get_shot(Coord {x: self.x, y: self.y}),
            _ => ()
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.ship {
            Some(_) => false,
            _ => true,
        }
    }

    pub fn symbol(&self) -> String {
        if self.shot {
            return "X".to_string();
        }
        match self.ship {
            Some(_) => {
                let rc: Rc<RefCell<ShipPiece>> = Rc::clone(&self.ship.as_ref().unwrap());
                let refcell = <Rc<RefCell<ShipPiece>> as std::borrow::Borrow<RefCell<ShipPiece>>>::borrow(&rc);
                format!(
                    "{}",
                    refcell.borrow().symbol()
                )
            },
            None => ".".to_string(),
        }
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