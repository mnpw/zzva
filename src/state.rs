use std::fmt::Display;

#[derive(Default, Clone)]
pub struct State {
    pub game_state: GameState,
    pub message: String,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Won,
    Lost,
    InProgress,
}

impl Default for GameState {
    fn default() -> Self {
        Self::InProgress
    }
}

#[derive(PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

impl From<&str> for Move {
    fn from(direction: &str) -> Self {
        let mv = match direction.to_ascii_lowercase().as_str() {
            "up" => Self::Up,
            "down" => Self::Down,
            "left" => Self::Left,
            "right" => Self::Right,
            _ => Self::Invalid,
        };

        mv
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            &Self::Up => "⬆",
            &Self::Down => "⬇",
            &Self::Left => "⬅",
            &Self::Right => "➡",
            _ => "Invalid move ❌",
        };

        write!(f, "{}", display)
    }
}
