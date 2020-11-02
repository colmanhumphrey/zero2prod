// use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, startup, telemetry};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .map_err(anyhow::Error::from)
        .with_context(|| "Failed to connect to Postgres.")?;
        .expect("Failed to connect to postgres");
        // .map_err(anyhow::Error::from)
        // .with_context(|| "Failed to connect to Postgres.")?;

    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection)?.await?;

    Ok(())
}
