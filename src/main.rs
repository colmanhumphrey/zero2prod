use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::{configuration, email_client, startup, telemetry};

#[actix_web::main]
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
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to postgres");

    let sender_email = configuration
        .email_client
        .sender()
        .expect("invalid sender email address");
    let email_client = email_client::EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
    );

    let listener = TcpListener::bind(address)?;
    startup::run(listener, connection_pool, email_client)?.await?;

    Ok(())
}
