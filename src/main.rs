use ::zero2prod::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgConnection::connect(&configuration.database_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection)?.await
}
