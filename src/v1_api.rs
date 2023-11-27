use crate::routes;
use actix_web::web;
use tracing::info;

pub fn v1_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuration v1 routes ...");
    cfg.service(
        web::resource("/things")
            .route(web::get().to(routes::get_things))
            .route(web::post().to(routes::post_things))
            .route(web::put().to(routes::put_things))
            .route(web::patch().to(routes::patch_things))
            .route(web::delete().to(routes::delete_things)),
    )
    .service(web::resource("/health_check").route(web::get().to(routes::get_health_check)));
}
