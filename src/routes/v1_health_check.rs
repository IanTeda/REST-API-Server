use actix_web::HttpResponse;

#[tracing::instrument] // Logging
pub async fn get_health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
