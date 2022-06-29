use rand::seq::SliceRandom;
use std::fmt::Display;

pub struct Board {
    size: usize,
    pub inner: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size,
            inner: vec![vec![Tile(0); size]; size],
        }
    }

    pub fn print(&self) {
        for row in &self.inner {
            for tile in row {
                print!("{} ", tile.0);
            }
            println!();
        }
    }

    fn collapse_inner(mut row: Vec<Tile>) -> Vec<Tile> {
        // collapse a row in <- direction
        // [2, 2, 4, 2] collapses in [4, 4, 2]

        // clean the row
        // remove all empty tiles

        row.reverse();

        let mut row = row
            .iter()
            .filter(|&n| n.0 != 0)
            .map(|n| n.clone())
            .collect::<Vec<Tile>>();

        let mut new_vec = Vec::new();
        while !row.is_empty() {
            let last = row.pop();
            let second_last = row.pop();

            if last.is_some() && second_last.is_some() {
                let last = last.unwrap();
                let second_last = second_last.unwrap();

                if last == second_last {
                    new_vec.push(Tile(last.0 + second_last.0));
                } else {
                    new_vec.push(last);
                    row.push(second_last);
                }
            } else if last.is_some() && second_last.is_none() {
                let last = last.unwrap();
                new_vec.push(last);
            }
        }

        new_vec
    }

    fn collapse(mut row: Vec<Tile>, direction: &str) -> Vec<Tile> {
        // println!("old row: {:?}", row);
        let size = row.len();
        let mut new_row: Vec<Tile> = Vec::new();

        if direction == "<-" {
            new_row = Self::collapse_inner(row);
            new_row.resize(size, Tile(0));
        } else if direction == "->" {
            row.reverse();
            new_row = Self::collapse_inner(row);
            new_row.resize(size, Tile(0));
            new_row.reverse();
        }

        // println!("new row: {:?}", new_row);
        new_row
    }

    pub fn play(&mut self, direction: &str) {
        // horizontal move
        // go row by row
        if direction == "left" || direction == "right" {
            for row in 0..self.size {
                // first impose gravity and move all elements
                let old_row = self.inner[row].clone();
                let mut new_row = vec![Tile(0); self.size];

                if direction == "left" {
                    new_row = Self::collapse(old_row, "<-");
                } else if direction == "right" {
                    new_row = Self::collapse(old_row, "->");
                }

                let _ = std::mem::replace(&mut self.inner[row], new_row);
                // remove all zeros from old_row
                // paste the old_row onto new_row depending on direciton
            }
        }

        // vertical move
        // go column by column
        if direction == "up" || direction == "down" {
            for col in 0..self.size {
                // first impose gravity and move all elements

                let mut old_row = Vec::new();
                for row in 0..self.size {
                    old_row.push(self.inner[row][col].clone());
                }

                let mut new_row = vec![Tile(0); self.size];

                if direction == "up" {
                    new_row = Self::collapse(old_row, "<-");
                } else if direction == "down" {
                    new_row = Self::collapse(old_row, "->");
                }

                for row in 0..self.size {
                    self.inner[row][col] = new_row[row].clone();
                }

                // remove all zeros from old_row
                // paste the old_row onto new_row depending on direciton
            }
        }
    }

    pub fn check(&mut self) {}

    pub fn spawn(&mut self) {
        let mut free_ids = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                if self.inner[row][col].0 == 0 {
                    free_ids.push((row, col));
                }
            }
        }

        if let Some(&pos) = free_ids.choose(&mut rand::thread_rng()) {
            self.inner[pos.0][pos.1] = Tile(2);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.inner {
            for tile in row {
                write!(f, "{} ", tile.0)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tile(usize);
