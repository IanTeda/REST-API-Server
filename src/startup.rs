use crate::v1_api;
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use std::net::TcpListener;
use tracing::info;
use tracing_actix_web;

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    // Start Actix web server
    let server = HttpServer::new(|| {
        App::new()
            // Add tracing (logging) to Actix web middleware
            .wrap(tracing_actix_web::TracingLogger::default())
            // Remove trailing of multiple '/'
            .wrap(middleware::NormalizePath::default())
            // Add version 1 API routes
            .service(web::scope("/api/v1").configure(v1_api::v1_routes))
    })
    .listen(listner)?
    .run();

    info!("Starting server");
    Ok(server)
}
