// use env_logger::Env;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use zero2prod::{configuration, startup, telemetry};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port,
    );
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    // .map_err(anyhow::Error::from)
    // .with_context(|| "Failed to connect to Postgres.")?;

    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool)?.await?;

    Ok(())
}
