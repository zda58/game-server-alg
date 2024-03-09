use serverinfo::data::{coord::Coord, gamestate::{CurrentGameState, CurrentState}};

use crate::{data::ship::Ship, gamedata::GameData};

pub struct GameView {
    p1_board: Vec<Vec<ViewCoord>>,
    p2_board: Vec<Vec<ViewCoord>>,
}

impl GameView {
    pub fn new(height: i32, width: i32) -> Self {
        GameView {
            p1_board: vec![vec![ViewCoord { state: State::Normal, piece: CoordPiece::Empty }; width as usize]; height as usize],
            p2_board: vec![vec![ViewCoord { state: State::Normal, piece: CoordPiece::Empty }; width as usize]; height as usize]
        }
    }

    pub fn populate_ships(&mut self, p1_ship_coords: &Vec<Ship>, p2_ship_coords: &Vec<Ship>) {
        for ship in p1_ship_coords {
            for coord in &ship.coords {
                self.p1_board[coord.y as usize][coord.x as usize].piece = CoordPiece::Ship;
            }
        }
        for ship in p2_ship_coords {
            for coord in &ship.coords {
                self.p2_board[coord.y as usize][coord.x as usize].piece = CoordPiece::Ship;
            }
        }
    }

    pub fn report_player_shots(&mut self, p1_shots: &Vec<Coord>, p2_shots: &Vec<Coord>) {
        for coord in p1_shots {
            self.p2_board[coord.y as usize][coord.x as usize].shoot_at();
        }
        for coord in p2_shots {
            self.p1_board[coord.y as usize][coord.x as usize].shoot_at();
        }
    }

    pub fn report_player_damaged_coords(&mut self, p1_damaged_coords: &Vec<Coord>, p2_damaged_coords: &Vec<Coord>) {
        for coord in p1_damaged_coords {
            self.p1_board[coord.y as usize][coord.x as usize].hit();
        }
        for coord in p2_damaged_coords {
            self.p2_board[coord.y as usize][coord.x as usize].hit();
        }
    }

    pub fn draw_view(&mut self, p1_shots_taken: i32, p2_shots_taken: i32) {
        let clear_screen = "\x1b[2J";
        let gray = "\x1b[38;5;242m";
        let orange = "\x1b[38;5;208m";
        let red = "\x1b[38;5;196m";
        let light_blue = "\x1b[38;5;153m";
        let block = "\u{2588}";
        let reset_color = "\x1b[0m";
        let indent = "\x1b[5C";

        print!("{}", clear_screen);
        let width: i32 = self.p1_board[0].len() as i32;
        print!("\x1b[{}C", 2 * width - 4);
        print!("Player 1");
        print!("\x1b[{}C", 4 * width + 4 - 8);
        print!("Player 2");
        println!();
        print!("\x1b[{}C", 2 * width - 4);
        print!("Shots: {}", p1_shots_taken);
        print!("\x1b[{}C", 4 * width + 4 - 8);
        print!("Shots: {}", p2_shots_taken);
        println!();
        println!();

        for i in 0..self.p1_board.len() {
            for j in 0..self.p1_board[i].len() {
                let coord_p1 = &self.p1_board[i][j];
                match coord_p1.piece {
                    CoordPiece::Empty => {
                        match coord_p1.state {
                            State::Hit => print!("{}", red),
                            State::Shot => print!("{}", orange),
                            State::Normal => print!("{}", gray),
                        }
                    },
                    CoordPiece::Ship => {
                        match coord_p1.state {
                            State::Hit => print!("{}", red),
                            _ => print!("{}", light_blue),
                        }
                    },
                }
                print!(" {}{} ", block, block);
            }
            print!("{}", indent);
            for j in 0..self.p1_board[i].len() {
                let coord_p2 = &self.p2_board[i][j];
                match coord_p2.piece {
                    CoordPiece::Empty => {
                        match coord_p2.state {
                            State::Hit => print!("{}", red),
                            State::Shot => print!("{}", orange),
                            State::Normal => print!("{}", gray),
                        }
                    },
                    CoordPiece::Ship => {
                        match coord_p2.state {
                            State::Hit => print!("{}", red),
                            _ => print!("{}", light_blue),
                        }
                    },
                }
                print!(" {}{} ", block, block);
            }
            println!("{}", reset_color);
            println!();
        }
    }


    pub fn draw_end_game(&mut self, p1_state: &CurrentState, p2_state: &CurrentState, data: &GameData) {
        let clear_screen = "\x1b[2J";
        let gray = "\x1b[38;5;242m";
        let orange = "\x1b[38;5;208m";
        let red = "\x1b[38;5;196m";
        let light_blue = "\x1b[38;5;153m";
        let block = "\u{2588}";
        let reset_color = "\x1b[0m";
        let indent = "\x1b[5C";

        print!("{}", clear_screen);
        let width: i32 = self.p1_board[0].len() as i32;
        print!("\x1b[{}C", 2 * width - 4);
        print!("Player 1");
        print!("\x1b[{}C", 4 * width + 4 - 8);
        print!("Player 2");
        println!();
        print!("\x1b[{}C", 2 * width - 2);
        match p1_state.current_state {
            CurrentGameState::Win => print!("Win!"),
            CurrentGameState::Loss => print!("Loss!"),
            CurrentGameState::Draw => print!("Draw!"),
            _ => (),
        }
        print!("\x1b[{}C", 4 * width + 4 - 4);
        match p2_state.current_state {
            CurrentGameState::Win => print!("Win!"),
            CurrentGameState::Loss => print!("Loss!"),
            CurrentGameState::Draw => print!("Draw!"),
            _ => (),
        }
        println!();
        println!();

        for i in 0..self.p1_board.len() {
            for j in 0..self.p1_board[i].len() {
                let coord_p1 = &self.p1_board[i][j];
                match coord_p1.piece {
                    CoordPiece::Empty => {
                        match coord_p1.state {
                            State::Hit => print!("{}", red),
                            State::Shot => print!("{}", orange),
                            State::Normal => print!("{}", gray),
                        }
                    },
                    CoordPiece::Ship => {
                        match coord_p1.state {
                            State::Hit => print!("{}", red),
                            _ => print!("{}", light_blue),
                        }
                    },
                }
                print!(" {}{} ", block, block);
            }
            print!("{}", indent);
            for j in 0..self.p1_board[i].len() {
                let coord_p2 = &self.p2_board[i][j];
                match coord_p2.piece {
                    CoordPiece::Empty => {
                        match coord_p2.state {
                            State::Hit => print!("{}", red),
                            State::Shot => print!("{}", orange),
                            State::Normal => print!("{}", gray),
                        }
                    },
                    CoordPiece::Ship => {
                        match coord_p2.state {
                            State::Hit => print!("{}", red),
                            _ => print!("{}", light_blue),
                        }
                    },
                }
                print!(" {}{} ", block, block);
            }
            println!("{}", reset_color);
            println!();
        }
    }
}

#[derive(Clone)]
struct ViewCoord {
    state: State,
    piece: CoordPiece
}

impl ViewCoord {
    pub fn shoot_at(&mut self) {
        match self.state {
            State::Normal => self.state = State::Shot,
            _ => (),
        }
    }

    pub fn hit(&mut self) {
        self.state = State::Hit;
    }
}

#[derive(Clone)]
enum State {
    Normal,
    Shot,
    Hit,
}

#[derive(Clone)]
enum CoordPiece {
    Empty,
    Ship
}