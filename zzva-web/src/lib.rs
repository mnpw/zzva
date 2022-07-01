use std::sync::Mutex;

use zzva::{game::Game, state::State};

pub mod routes;

pub struct AppState {
    pub game: Mutex<Option<Game>>,
    pub state: Mutex<Option<State>>,
}

impl AppState {
    pub fn get_game(&self) -> std::sync::MutexGuard<'_, Option<Game>> {
        self.game.lock().unwrap()
    }

    pub fn get_state(&self) -> std::sync::MutexGuard<'_, Option<State>> {
        self.state.lock().unwrap()
    }
}

impl Default for AppState {
    fn default() -> Self {
        let game = Mutex::new(None);
        let state = Mutex::new(None);

        Self { game, state }
    }
}
