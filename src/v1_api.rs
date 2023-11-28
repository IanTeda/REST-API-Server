use crate::routes;
use actix_web::web;
use tracing::info;

pub fn v1_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuration v1 routes ...");
    cfg.service(
        web::scope("/things")
            .service(routes::things_get)
            .service(routes::things_post)
            .service(routes::things_put)
            .service(routes::things_patch)
            .service(routes::things_delete),
    );
    cfg.service(web::scope("/ping").service(routes::ping_get));
}
