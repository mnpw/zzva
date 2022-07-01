use std::fmt::Display;

use crate::{
    board::*,
    state::{GameState, Move, State},
    tile::Tile,
};

#[derive(Clone)]
pub struct Game {
    _size: usize,
    board: Board,
    win_tile: Tile,
    state: State,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Game {
    pub fn new(size: usize, max_tile: usize) -> Result<Self, &'static str> {
        let mut board = Board::new(size);
        board.spawn()?;
        board.spawn()?;

        Ok(Game {
            _size: size,
            board,
            win_tile: Tile::new(max_tile),
            state: Default::default(),
        })
    }

    pub fn from(size: usize, max_tile: usize, state: &str) -> Self {
        let board = Board::from_state(size, state);

        Game {
            _size: size,
            board,
            win_tile: Tile::new(max_tile),
            state: Default::default(),
        }
    }

    pub fn play(&mut self, direction: &str) -> Result<State, &'static str> {
        let mut message = String::new();

        let mv = Move::from(direction);
        message.push_str(&format!("You chose: {}\n", mv));

        // Play the move
        if let Err(e) = self.board.play(&mv) {
            return Err(e);
        }

        // Evaluate the game state
        let game_state = self.board.check(&self.win_tile).unwrap();
        let status = match game_state {
            GameState::Won => "You Won! Game finished. 🥳🥂",
            GameState::Lost => "You Lost! :(",
            GameState::InProgress => match self.board.spawn() {
                Ok(()) => "",
                Err(e) => return Err(e),
            },
        };
        message.push_str(&status);

        self.state = State {
            game_state,
            message,
        };

        Ok(self.state.clone())
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    #[cfg(test)]
    fn spawn(&mut self) {
        self.board.check(&self.win_tile).unwrap();
    }

    #[cfg(test)]
    fn check(&mut self) -> Result<GameState, std::convert::Infallible> {
        self.board.check(&self.win_tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn in_progress() {
        let state = indoc! {"
        0,0,0,2
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);
    }

    #[test]
    fn lost() {
        let state = indoc! {"
        2,4,2,4
        4,2,4,2
        2,4,2,4
        4,2,4,2
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Lost);
    }

    #[test]
    fn won() {
        let state = indoc! {"
        2048,0,0,2
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn won_full_board() {
        let state = indoc! {"
        2,4,2,4
        4,2,8,2
        2,4,2,4
        4,2,4,2
    "};

        let mut game = Game::from(4, 8, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn next_move_win_top_left() {
        let state = indoc! {"
        1024,1024,0,0
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);

        game.play("left").unwrap();
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn next_move_win_top_right() {
        let state = indoc! {"
        0,0,1024,1024
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);

        game.play("right").unwrap();
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn next_move_win_bottom_left() {
        let state = indoc! {"
        0,0,0,0
        0,0,0,0
        0,0,0,0   
        1024,1024,0,0
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);

        game.play("left").unwrap();
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn next_move_win_bottom_right() {
        let state = indoc! {"
        0,0,0,0
        0,0,0,0
        0,0,0,0   
        0,0,1024,1024
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);

        game.play("right").unwrap();
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn next_move_win_center() {
        let state = indoc! {"
        0,0,0,0
        0,0,0,0
        0,1024,1024,0
        0,0,0,0   
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);

        game.play("left").unwrap();
        let state = game.check().unwrap();
        assert_eq!(state, GameState::Won);
    }

    #[test]
    fn alter_winning_number() {
        let state = indoc! {"
        1024,1024,0,0
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

        let mut game = Game::from(4, 4096, state);
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);

        game.play("left").unwrap();
        let state = game.check().unwrap();
        assert_eq!(state, GameState::InProgress);
    }

    #[test]
    fn next_move_lose() {
        let state = indoc! {"
        8,16,8,16
        16,8,16,8
        8,16,8,16
        8,16,8,0
    "};

        let mut game = Game::from(4, 2048, state);
        let state = game.check().unwrap();

        assert_eq!(state, GameState::InProgress);

        game.play("right").unwrap();
        game.spawn();
        let state = game.check().unwrap();

        assert_eq!(state, GameState::Lost);
    }
}
