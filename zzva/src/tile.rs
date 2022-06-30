use std::{
    fmt::Display,
    ops::{Add, Deref},
};

use rand::Rng;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Tile(usize);

impl Tile {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            return Tile::default();
        }

        let is_even = !(size & (size - 1)) == 0;
        if is_even {
            panic!("Cannot create a tile without power of 2.")
        }

        Tile(size)
    }

    pub fn random() -> Self {
        // returns a tile of size 2 or 4
        // in probability 5:1

        let rng = rand::thread_rng().gen::<f64>() * 0.6;
        if rng > 0.5 {
            return Tile::new(4);
        } else {
            return Tile::new(2);
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Tile::default()
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Deref for Tile {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add for Tile {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tile(self.0 + other.0)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{: ^5}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_empty_means_default() {
        // a simple check never harms

        let t = Tile::default();
        assert!(t.is_empty())
    }

    #[test]
    fn check_tile_display() {
        let t = Tile::new(2);
        assert_eq!("  2  ", t.to_string());

        let t = Tile::new(16);
        assert_eq!(" 16  ", t.to_string());

        let t = Tile::new(2048);
        assert_eq!("2048 ", t.to_string());
    }
}
