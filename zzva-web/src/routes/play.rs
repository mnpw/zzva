use actix_web::{web, HttpResponse};
use serde::Deserialize;
use zzva::state::GameState;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct Move {
    direction: String,
}

pub async fn play(form: web::Form<Move>, app: web::Data<AppState>) -> HttpResponse {
    let mv = &form.direction;
    match app.get_game().as_mut() {
        Some(game) => match game.play(mv) {
            Ok(res) => {
                *app.get_state() = Some(res.clone());
                match res.game_state {
                    GameState::Won | GameState::Lost => app.reset_game(),
                    GameState::InProgress => todo!(),
                }
            }
            Err(_) => {
                return HttpResponse::InternalServerError().body("Could not play the move!");
            }
        },
        None => {
            return HttpResponse::BadRequest().body("Game is uninitialized!");
        }
    };

    HttpResponse::Ok().finish()
}
