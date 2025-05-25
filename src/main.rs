use actix_web::{App, HttpServer};
mod models;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .init();

    HttpServer::new(move || {
        App::new()
            .configure(api::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
