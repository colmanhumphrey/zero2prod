use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::startup;
use env_logger::Env;

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection)?.await?;

    Ok(())
}
