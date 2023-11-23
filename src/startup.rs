use crate::routes::health_check; // crate refers to src/lib.rs
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing::{error, info, Level};
use tracing_actix_web;

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    // Start Actix web server
    let server = HttpServer::new(|| {
        App::new()
            // Add tracing to Actix web middleware
            .wrap(tracing_actix_web::TracingLogger::default())
            // Health check rout
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listner)?
    .run();
    info!("Starting server");
    Ok(server)
}
