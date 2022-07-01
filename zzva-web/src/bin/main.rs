use actix_web::{web, App, HttpServer};
use zzva_web::{
    routes::{health_check, play, start, start_default, view},
    state::AppState,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .app_data(app.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/start", web::post().to(start))
            .route("/start_default", web::post().to(start_default))
            .route("/play", web::post().to(play))
            .route("/view", web::get().to(view))
            .route("/view_raw", web::get().to(view))
    })
    .bind("0:8080")?
    .run()
    .await
}
