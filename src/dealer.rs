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
    pub fn run(&mut self) {
        let mut count: u32 = 0;

        let mut ships: HashMap<ShipType, u32> = HashMap::from ([
            (ShipType::Submarine, 3),
            (ShipType::Destroyer, 3),
            (ShipType::Battleship, 3),
            (ShipType::Carrier, 3)
        ]);
        
        let mut game_state = GameState::Ongoing;
        
        while game_state == GameState::Ongoing {
            let p1shots: Vec<Coord> = self.player1.take_shots();
            let p2shots : Vec<Coord> = self.player2.take_shots();

            let p1hits: Vec<Coord> = self.player2.report_damage(p1shots);
            let p2hits: Vec<Coord> = self.player1.report_damage(p2shots);
            self.player1.record_successful_hits(p1hits);
            self.player2.record_successful_hits(p2hits);
            if self.player1.get_ship_count() == 0 && self.player2.get_ship_count() == 0 {
                game_state = GameState::Draw;
            } else if self.player1.get_ship_count() == 0 {
                game_state = GameState::P2Win;
            } else if self.player2.get_ship_count() == 0 {
                game_state = GameState::P1Win;
            }
        }
        match game_state {
            GameState::Draw => println!("Draw game!"),
            GameState::P1Win => println!("Player 1 wins!"),
            GameState::P2Win => println!("Player 2 wins!"),
            _ => println!("some error occured"),
        }
        
    }
}

fn print_board(board: &Vec<Vec<OwnCoord>>) {
    let height = board.len();
    let width = board[0].len();
    for y in 0..height {
        for x in 0..width {
            match &board[y][x].ship {
                Some(cell) => print!(" {} ", cell.borrow_mut().ship_type.symbol()),
                None => print!(" n ")
            }
        }
        println!()
    }
}