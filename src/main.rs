use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting zero2prod application...");

    log::info!("Reading configuration...");
    let configuration = get_configuration().expect("Failed to read configuration.");

    log::info!("Connecting to Postgres...");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    log::info!("Starting application at: {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
