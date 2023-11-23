use actix_web::HttpResponse;

#[tracing::instrument] // Logging
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
