pub mod template;

// Use this file to add helper functions and additional modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord(i32, i32);

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord(x, y)
    }
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
    pub fn up(&self) -> Self {
        Coord(self.0, self.1 - 1)
    }
    pub fn down(&self) -> Self {
        Coord(self.0, self.1 + 1)
    }
    pub fn left(&self) -> Self {
        Coord(self.0 - 1, self.1)
    }
    pub fn right(&self) -> Self {
        Coord(self.0 + 1, self.1)
    }
    pub fn neighbors(&self) -> [Self; 8] {
        [
            self.up(),
            self.down(),
            self.left(),
            self.right(),
            self.up().left(),
            self.up().right(),
            self.down().left(),
            self.down().right(),
        ]
    }
}
impl<T: Into<i32>> From<(T, T)> for Coord {
    fn from(tuple: (T, T)) -> Self {
        Coord(tuple.0.into(), tuple.1.into())
    }
}

impl From<Coord> for (i32, i32) {
    fn from(coord: Coord) -> Self {
        (coord.0, coord.1)
    }
}
