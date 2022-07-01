use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct Move {
    direction: String,
}

pub async fn play(form: web::Form<Move>, app: web::Data<AppState>) -> HttpResponse {
    let mv = &form.direction;
    match app.get_game().as_mut() {
        Some(game) => match game.play(mv) {
            Ok(res) => {
                *app.get_state() = Some(res);
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
