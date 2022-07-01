use actix_web::{web, HttpResponse};
use serde::Deserialize;
use zzva::game::Game;

use crate::AppState;

#[derive(Deserialize)]
pub struct Config {
    board: u8,
    max: u16,
}

pub async fn start_default(app: web::Data<AppState>) -> HttpResponse {
    let game = Game::new(4, 2048);

    let app_game = game.unwrap();

    *app.get_game() = Some(app_game.clone());
    *app.get_state() = Some(app_game.get_state().clone());

    HttpResponse::Ok().finish()
}

pub async fn start(form: web::Form<Config>, app: web::Data<AppState>) -> HttpResponse {
    let game = Game::new(form.board.into(), form.max.into());

    let app_game = game.unwrap();

    *app.get_game() = Some(app_game.clone());
    *app.get_state() = Some(app_game.get_state().clone());

    HttpResponse::Ok().finish()
}
