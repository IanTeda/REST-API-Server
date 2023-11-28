///////////////////////////////////////////////////////////////////////////////
// PING ROUTE
// Route to check the API is up and running
use actix_web::{get, web, Error, HttpResponse};
use futures::{future::ok, stream::once};

#[tracing::instrument(name = "Ping GET")]
#[get("")]
pub async fn ping_get() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"Pong!")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
