use actix_server::Server;
use actix_web::{App, HttpServer};
use middleware::ErrorLogging;
use routes::{get_status, persona_webhook_handler, unlimit_webhook_handler};

mod db;
pub mod error;
mod middleware;
pub(crate) mod routes;

pub const PORT: u16 = 9999;

pub async fn create_server() -> eyre::Result<Server> {
    env_logger::init();
    Ok(HttpServer::new(|| {
        App::new()
            .wrap(ErrorLogging)
            .service(get_status)
            .service(unlimit_webhook_handler)
            .service(persona_webhook_handler)
    })
    .bind(("0.0.0.0", PORT))?
    .run())
}
