use std::cmp::{max, min};

pub mod template;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord(i32, i32);

impl Coord {
    /// Create a new coordinate given x and y values
    pub fn new(x: i32, y: i32) -> Self {
        Coord(x, y)
    }

    /// Returns x portion of coordinate
    #[inline]
    pub fn x(&self) -> i32 {
        self.0
    }

    /// Returns y portion of coordinate
    #[inline]
    pub fn y(&self) -> i32 {
        self.1
    }

    /// Return a new coordinate, shifted up
    #[inline]
    pub fn up(&self) -> Self {
        Coord(self.0, self.1 - 1)
    }

    /// Return a new coordinate, shifted down
    #[inline]
    pub fn down(&self) -> Self {
        Coord(self.0, self.1 + 1)
    }

    /// Return a new coordinate, shifted to the left
    #[inline]
    pub fn left(&self) -> Self {
        Coord(self.0 - 1, self.1)
    }

    /// Return a new coordinate, shifted to the right
    #[inline]
    pub fn right(&self) -> Self {
        Coord(self.0 + 1, self.1)
    }

    /// Get all 8 neighboring coordinates
    #[inline]
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

/// Condense overlapping and contiguous ranges into minimal set of ranges
///
/// For example, 1-3, 2-5 becomes 1-5
pub fn condense_ranges(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    let mut condensed: Vec<(usize, usize)> = Vec::new();

    for &(start, end) in &sorted_ranges {
        if let Some(&mut (_, ref mut last_end)) = condensed.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                condensed.push((start, end));
            }
        } else {
            condensed.push((start, end));
        }
    }

    condensed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord3(usize, usize, usize);

impl Coord3 {
    /// Create a new 3D coordinate given x, y, and z values
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Coord3(x, y, z)
    }

    /// Returns x portion of coordinate
    #[inline]
    pub fn x(&self) -> usize {
        self.0
    }

    /// Returns y portion of coordinate
    #[inline]
    pub fn y(&self) -> usize {
        self.1
    }

    /// Returns z portion of coordinate
    #[inline]
    pub fn z(&self) -> usize {
        self.2
    }

    #[inline]
    // strangely, keeping the sqrt improves performance
    // TODO: check out what the compiler does here
    pub fn distance(&self, other: &Coord3) -> u32 {
        ((self.x().abs_diff(other.x()).pow(2)
            + self.y().abs_diff(other.y()).pow(2)
            + self.z().abs_diff(other.z()).pow(2)) as f32)
            .sqrt() as u32
    }
}

impl From<(usize, usize, usize)> for Coord3 {
    fn from(value: (usize, usize, usize)) -> Self {
        Coord3(value.0, value.1, value.2)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rectangle {
    pub top_left: Coord,
    pub bottom_right: Coord,
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        dbg!(self);
        let width = self.top_left.x().max(self.bottom_right.x())
            - self.top_left.x().min(self.bottom_right.x());
        let height = self.top_left.y().max(self.bottom_right.y())
            - self.top_left.y().min(self.bottom_right.y());
        dbg!(self.top_left.y(), self.bottom_right.y());
        dbg!(width, height, width * height);
        width * height
    }

    pub fn new(first: Coord, second: Coord) -> Self {
        let top_left = Coord::from((first.x().min(second.x()), first.y().min(second.y())));
        let bottom_right = Coord::from((first.x().max(second.x()), first.y().max(second.y())));
        Rectangle {
            top_left,
            bottom_right,
        }
    }

    pub fn contains(&self, point: &Coord) -> bool {
        point.x() >= self.top_left.x()
            && point.x() <= self.bottom_right.x()
            && point.y() >= self.top_left.y()
            && point.y() <= self.bottom_right.y()
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.top_left.x() > other.bottom_right.x()
            || other.top_left.x() > self.bottom_right.x()
            || self.top_left.y() > other.bottom_right.y()
            || other.top_left.y() > self.bottom_right.y()
        {
            None
        } else {
            Some(Rectangle {
                top_left: Coord::from((
                    max(self.top_left.x(), other.top_left.x()),
                    max(self.top_left.y(), other.top_left.y()),
                )),
                bottom_right: Coord::from((
                    min(self.bottom_right.x(), other.bottom_right.x()),
                    min(self.bottom_right.y(), other.bottom_right.y()),
                )),
            })
        }
    }

    pub fn inset(&self, offset: i32) -> Self {
        Rectangle::new(
            Coord::from((self.top_left.x() + offset, self.top_left.y() + offset)),
            Coord::from((
                self.bottom_right.x() - offset,
                self.bottom_right.y() - offset,
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_area() {
        let rect = Rectangle::new((0, 0).into(), (10, 10).into());
        assert_eq!(rect.area(), 100);
    }
}
