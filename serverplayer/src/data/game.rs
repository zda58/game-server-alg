#[derive(PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    P1Win,
    P2Win,
    Draw
}