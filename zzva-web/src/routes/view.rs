use actix_web::{web, HttpResponse};

use crate::AppState;

pub async fn view(app: web::Data<AppState>) -> HttpResponse {
    match app.get_game().as_mut() {
        Some(game) => {
            return HttpResponse::Ok().body(game.get_board().to_string());
        }
        None => {
            return HttpResponse::BadRequest().body("Game is uninitialized!");
        }
    };
}
