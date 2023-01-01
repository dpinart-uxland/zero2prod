use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::{get_configuration, Settings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    initialize_logging();
    let configuration = get_configuration().expect("Failed to read configuration file.");
    let connection = get_db_connection(&configuration).await;
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}

async fn get_db_connection(settings: &Settings) -> PgPool {
    let db_pool = PgPool::connect(settings.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to postgres");
    db_pool
}

///Initializes logging for the application
fn initialize_logging() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
}
