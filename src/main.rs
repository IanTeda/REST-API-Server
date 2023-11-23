use rest_api_server::configuration::get_configuration;
use rest_api_server::startup::run;
use rest_api_server::telemetry;
use std::net::TcpListener;
use tracing::{error, info, Level};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up tracing subscriber
    let subscriber = telemetry::get_subscriber();
    // Initiate tracing
    telemetry::init_subscriber(subscriber);

    // Get configuration
    let configuration = get_configuration().expect("Failed to read configuration.toml");

    // Set server address from config
    let address = format!("127.0.0.1:{}", configuration.server_port);

    // Set TCP listener
    let listener = TcpListener::bind(address)?;

    // Run Actix server
    run(listener)?.await
}
