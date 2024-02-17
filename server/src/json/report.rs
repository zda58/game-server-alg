pub struct Report {
    shots_hit: Vec<Coord>,
    coords_damaged: Vec<Coord>
}

struct Coord {
    x: i32,
    y: i32
}