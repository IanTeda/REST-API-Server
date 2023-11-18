use rest_api_server::startup::run;
use std::net::TcpListener;

fn spawn_app() -> String {
    // The host will randomly assign an unused port when binding to "0"
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Retrieve the port assigned by host
    let _port = listener.local_addr().unwrap().port();

    // Spawn server
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // We return the application address to the caller!
    format!("http://127.0.0.1:{}", _port)
}

// #[tokio::test]
#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
