use log::{debug, info, warn};
use rand::seq::SliceRandom;
use serde::Serialize;
use std::{convert::Infallible, fmt::Display, ops::Deref};

use crate::{
    state::{GameState, Move},
    tile::*,
};

#[derive(Clone, Serialize)]
pub struct Board {
    size: usize,
    inner: Vec<Vec<Tile>>,
    state: GameState,
}

impl Board {
    /// Create a board from a give size and state in string form.
    ///
    /// The state should look like:
    /// 0,8,0,0
    /// 0,0,2,0
    /// ..
    /// 0,4,0,0
    pub fn from_state(size: usize, state: &str) -> Self {
        let mut inner = vec![vec![Tile::default(); size]; size];

        for (i, line) in state.lines().enumerate() {
            for (j, tile) in line.split(",").enumerate() {
                let tile: usize = tile.trim().parse().unwrap();
                inner[i][j] = Tile::new(tile);
            }
        }

        info!("Game loaded!");
        Board {
            size,
            inner,
            state: GameState::InProgress,
        }
    }

    /// Create a board from a given size.
    pub fn new(size: usize) -> Self {
        info!("Game created!");
        Board {
            size,
            inner: vec![vec![Tile::default(); size]; size],
            state: GameState::InProgress,
        }
    }

    /// Execute a move on the board on the chosen direction.
    ///
    /// For the following board and move direction 'Up',
    /// 0 0 0 0
    /// 0 0 0 0
    /// 0 2 0 0
    /// 0 0 0 0
    /// the board changes to:
    /// 0 2 0 0
    /// 0 0 0 0
    /// 0 0 0 0
    /// 0 0 0 0
    pub fn play(&mut self, direction: &Move) -> Result<(), &'static str> {
        info!("[start] Play - move chosen: {}", direction);
        let mut any_tiles_combined = false;

        match *direction {
            Move::Up | Move::Down => {
                for col in 0..self.size {
                    // extract each column
                    let old_col = self.get_col(col);

                    // collapse the column in move direction
                    let new_col = if *direction == Move::Up {
                        Self::collapse(&old_col, "<-")
                    } else {
                        Self::collapse(&old_col, "->")
                    };

                    debug!(
                        "id: {} | move: {} | Column collapsed {:?} -> {:?}",
                        col, direction, old_col, new_col
                    );

                    // check if move did anyting
                    any_tiles_combined = any_tiles_combined || old_col != new_col;

                    // replace old column by collapsed column
                    for row in 0..self.size {
                        self.inner[row][col] = new_col[row].clone();
                    }
                }
            }
            Move::Left | Move::Right => {
                for row in 0..self.size {
                    // extract each column
                    let old_row = self.get_row(row);

                    // collapse the column in move direction
                    let new_row = if *direction == Move::Left {
                        Self::collapse(&old_row, "<-")
                    } else {
                        Self::collapse(&old_row, "->")
                    };

                    debug!(
                        "id: {} | move: {} | Row collapsed {:?} -> {:?}",
                        row, direction, old_row, new_row
                    );

                    // check if move did anyting
                    any_tiles_combined = any_tiles_combined || old_row != new_row;

                    // replace old column by collapsed column
                    let _ = std::mem::replace(&mut self.inner[row], new_row);
                }
            }
            Move::Invalid => return Err("Invalid move."),
        }

        info!("[end] Play - move chosen: {}", direction);
        if !any_tiles_combined {
            return Err("Invalid move. Nothing happened.");
        }
        Ok(())
    }

    /// Walk through the board and determine game state.
    pub fn check(&mut self, win_tile: &Tile) -> Result<GameState, Infallible> {
        info!("[start] Check - win tile: {}", win_tile);
        let mut have_won = false;
        let mut have_lost = true;

        // check if won
        for i in 0..self.size {
            for j in 0..self.size {
                debug!(
                    "Checking tile: {} | win tile: {}",
                    self.inner[i][j], win_tile
                );
                if self.inner[i][j] == *win_tile {
                    have_won = true;
                    have_lost = false;
                    break;
                }
            }
        }

        if have_won {
            info!("[end] Check - win tile: {}", win_tile);
            self.state = GameState::Won;
            return Ok(self.state);
        }

        // check if lost
        for i in 0..self.size {
            for j in 0..self.size {
                // check if can move up
                if i as i32 - 1 > 0 {
                    if self.inner[i - 1][j].is_empty() || self.inner[i - 1][j] == self.inner[i][j] {
                        have_lost = false;
                        break;
                    }
                }

                // check if can move right
                if j as i32 + 1 < self.size as i32 {
                    if self.inner[i][j + 1].is_empty() || self.inner[i][j + 1] == self.inner[i][j] {
                        have_lost = false;
                        break;
                    }
                }

                // check if can move down
                if i as i32 + 1 < self.size as i32 {
                    if self.inner[i + 1][j].is_empty() || self.inner[i + 1][j] == self.inner[i][j] {
                        have_lost = false;
                        break;
                    }
                }

                // check if can move left
                if j as i32 - 1 > 0 {
                    if self.inner[i][j - 1].is_empty() || self.inner[i][j - 1] == self.inner[i][j] {
                        have_lost = false;
                        break;
                    }
                }
            }
        }

        if have_lost {
            info!("[end] Check - win tile: {}", win_tile);
            self.state = GameState::Lost;
            return Ok(self.state);
        }

        Ok(self.state)
    }

    /// Spawn a new tile on the board.
    pub fn spawn(&mut self) -> Result<(), &'static str> {
        info!("[start] Spawn");
        let mut free_ids = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                if self.inner[row][col].is_empty() {
                    free_ids.push((row, col));
                }
            }
        }

        if let Some(&pos) = free_ids.choose(&mut rand::thread_rng()) {
            info!("[end] Spawn");
            self.inner[pos.0][pos.1] = Tile::random();
            return Ok(());
        } else {
            warn!("[end] Spawn");
            return Err("No position available for spawn.");
        }
    }

    pub fn get_raw_board_state(&self) -> String {
        let mut board = String::new();

        for row in &self.inner {
            for tile in row {
                board.push_str(&tile.deref().to_string());
                board.push(',');
            }
            board.push('\n');
        }

        board
    }

    /// Collapse a row of tiles in right to left direction.
    ///
    /// [2, 2, 4, 2] collapses in [4, 4, 2]
    fn collapse_inner(mut row: Vec<Tile>) -> Vec<Tile> {
        row.reverse();

        // remove all empty tiles
        let mut row = row
            .iter()
            .filter(|&n| *n != Tile::default())
            .map(|n| n.clone())
            .collect::<Vec<Tile>>();

        let mut new_vec = Vec::new();

        // collapse tiles into one by taking two at a time
        while !row.is_empty() {
            let last = row.pop();
            let second_last = row.pop();

            if last.is_some() && second_last.is_some() {
                let last = last.unwrap();
                let second_last = second_last.unwrap();

                if last == second_last {
                    new_vec.push(Tile::new(*last + *second_last));
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

    /// Collapse a row in chosen direction.
    ///
    /// [2, 2, 4, 2] collapses in [4, 4, 2, 0] in left direction.
    /// [2, 2, 4, 2] collapses in [0, 4, 4, 2] in right direction.
    fn collapse(row: &Vec<Tile>, direction: &str) -> Vec<Tile> {
        let mut row = row.clone();
        let size = row.len();
        let mut new_row: Vec<Tile> = Vec::new();

        if direction == "<-" {
            new_row = Self::collapse_inner(row);
            new_row.resize(size, Tile::default());
        } else if direction == "->" {
            row.reverse();
            new_row = Self::collapse_inner(row);
            new_row.resize(size, Tile::default());
            new_row.reverse();
        }

        new_row
    }

    fn get_row(&self, id: usize) -> Vec<Tile> {
        self.inner[id].clone()
    }

    fn get_col(&self, id: usize) -> Vec<Tile> {
        let mut column = Vec::new();
        for row in 0..self.size {
            column.push(self.inner[row][id]);
        }

        column
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_width = 6 * self.size + 1;
        let line_separator = "=".repeat(line_width);

        writeln!(f, "{}", line_separator)?;
        for row in &self.inner {
            for tile in row {
                write!(f, "|{}", tile)?;
            }
            writeln!(f, "|\n{}", line_separator)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tile_row(input: &str) -> Vec<Tile> {
        let mut row: Vec<Tile> = Vec::new();

        for char in input.split(',') {
            row.push(Tile::new(
                char.trim()
                    .parse::<usize>()
                    .expect("Invalid tile row input."),
            ));
        }

        row
    }

    #[test]
    fn test_collapse_left() {
        let row = create_tile_row("2,2,4,2");
        let row = Board::collapse(&row, "<-");

        assert_eq!(row, create_tile_row("4,4,2,0"));
    }

    #[test]
    fn test_collapse_right() {
        let row = create_tile_row("2,2,4,2");
        let row = Board::collapse(&row, "->");

        assert_eq!(row, create_tile_row("0,4,4,2"));
    }
}
