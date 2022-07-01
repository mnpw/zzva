use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn status(app: web::Data<AppState>) -> HttpResponse {
    match app.get_state().as_mut() {
        Some(state) => {
            return HttpResponse::Ok().body(state.game_state.to_string());
        }
        None => {
            return HttpResponse::BadRequest().body("Game is uninitialized!");
        }
    };
}
