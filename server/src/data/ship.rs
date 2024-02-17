use super::coord::Coord;

pub struct Ship {
    coords: Vec<Coord>,
    destroyed_coords: Vec<Coord>,
    reported_hit_coords: Vec<Coord>,
}

impl Ship {
    pub fn new(coords: Vec<Coord>) -> Self {
        Self {
            coords: coords,
            destroyed_coords: Vec::new(),
            reported_hit_coords: Vec::new()
        }
    }

    pub fn shoot_at(&mut self, coord: Coord) {
        if self.coords.contains(&coord) {
            self.destroyed_coords.push(coord.clone());
            self.reported_hit_coords.push(coord);
        }
    }

    pub fn get_hit_coords(&mut self) -> Vec<Coord> {
        let report_vec: Vec<Coord> = self.reported_hit_coords.clone();
        self.reported_hit_coords.clear();
        report_vec
    }
}