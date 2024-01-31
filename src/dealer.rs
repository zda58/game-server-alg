use std::collections::HashMap;
use crate::data::coordinates::owncoord::OwnCoord;
use crate::data::ship::shippiece::ShipPiece;
use crate::player::algorithmplayer::{AlgorithmPlayer};
use crate::data::ship::shiptype::{ShipType};

pub struct Dealer {
    pub player1: AlgorithmPlayer,
    pub player2: AlgorithmPlayer
}

impl Dealer {
    pub fn run(&self) {
        let mut ships: HashMap<ShipType, u32> = HashMap::from ([
            (ShipType::SUBMARINE, 3),
            (ShipType::DESTROYER, 3),
            (ShipType::BATTLESHIP, 3),
            (ShipType::CARRIER, 3)
        ]);
        
        print_board(&self.player1.ownBoard);
        //let player1Ships: Vec<ShipPiece> = self.player1.setup();
    }
}

fn print_board(board: &Vec<Vec<OwnCoord>>) {
    let width = board.len();
    let height = board[0].len();
    for y in 0..height {
        for x in 0..width {
            match &board[x][y].ship {
                Some(cell) => print!(" {} ", cell.borrow_mut().shipType.symbol()),
                None => print!(" 0 ")
            }
        }
        println!()
    }
}