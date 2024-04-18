use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Reading configuration...");
    let configuration = get_configuration().expect("Failed to read configuration.");

    println!("Connecting to Postgres...");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("Starting application at: {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
