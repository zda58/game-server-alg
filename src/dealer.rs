use std::collections::HashMap;
use crate::data::coordinates::coord::Coord;
use crate::data::coordinates::owncoord::OwnCoord;
use crate::data::ship::shippiece::ShipPiece;
use crate::player::algorithmplayer::{AlgorithmPlayer};
use crate::data::ship::shiptype::{ShipType};
use crate::data::game::GameState;

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
        println!();
        print_board(&self.player2.ownBoard);
        //let player1Ships: Vec<ShipPiece> = self.player1.setup();
        let game_state = GameState::ONGOING;
        while game_state == GameState::ONGOING {
            let p1shots: Vec<Coord> = self.player1.take_shots();
            let p2shots : Vec<Coord> = self.player2.take_shots();

            let p1hits: Vec<Coord> = self.player2.report_damage(p1shots);
            let p2hits: Vec<Coord> = self.player1.report_damage(p2shots);
            self.player1.record_hits(p1hits);
            self.player2.record_hits(p2hits);

            if self.player1.get_ship_count() == 0 && self.player2.get_ship_count() == 0 {
                game_state == GameState::DRAW;
            } else if self.player1.get_ship_count() == 0 {
                game_state == GameState::P2WIN;
            } else if self.player2.get_ship_count() == 0 {
                game_state == GameState::P1WIN;
            }
        }
        match game_state {
            GameState::DRAW => println!("Draw game!"),
            GameState::P1WIN => println!("Player 1 wins!"),
            GameState::P2WIN => println!("Player 2 wins!"),
            _ => ()
        }
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