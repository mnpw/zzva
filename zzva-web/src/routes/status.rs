use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn status(app: web::Data<AppState>) -> HttpResponse {
    match app.get_game().as_mut() {
        Some(game) => {
            return HttpResponse::Ok().body(game.get_state().game_state.to_string());
        }
        None => {
            return HttpResponse::BadRequest().body("Game is uninitialized!");
        }
    };
}
