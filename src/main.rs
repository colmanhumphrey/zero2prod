use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::startup;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection)?.await
}
