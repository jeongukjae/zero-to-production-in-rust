use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
