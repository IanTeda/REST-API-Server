use rest_api_server::configuration::get_configuration;
use rest_api_server::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get configuration
    let configuration = get_configuration().expect("Failed to read configuration.toml");

    let address = format!("127.0.0.1:{}", configuration.server_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
