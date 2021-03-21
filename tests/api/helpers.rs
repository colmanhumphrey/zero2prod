use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wiremock::MockServer;
use zero2prod::{configuration, startup, telemetry};

lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { "" };
        let subscriber = telemetry::get_subscriber("test".into(), filter.into());
        telemetry::init_subscriber(subscriber);
    };
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub email_server: MockServer,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request")
    }
}

// used to not be pub
pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    // Stand-in for Postmark's API
    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = configuration::get_configuration().expect("Failed to read configuration.");
        // Use a different db for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use random OS port
        c.application.port = 0;
        // Mock server as email API
        c.email_client.base_url = email_server.uri();
        c
    };

    configure_database(&configuration.database).await;

    let application = startup::Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: startup::get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to the database"),
        email_server,
    }
}

// used to be pub
async fn configure_database(config: &configuration::DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
