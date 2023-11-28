///////////////////////////////////////////////////////////////////////////////
/// THING ROUTE
/// A template for creating a rest route
///
// method_route!(get, GET);
// method_route!(post, POST);
// method_route!(put, PUT);
// method_route!(patch, PATCH);
// method_route!(delete, DELETE);
// method_route!(head, HEAD);
// method_route!(trace, TRACE);
use actix_web::{delete, get, patch, post, put, web, Error, HttpResponse};
use futures::{future::ok, stream::once};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    thing_id: u32,
    query: String,
}

#[tracing::instrument(name = "Get tihngs")]
#[get("")]
pub async fn things_get() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"GET: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Put tihngs")]
#[post("")]
pub async fn things_post() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"POST: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Put tihngs")]
#[put("")]
pub async fn things_put() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"PUT: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Patch tihngs")]
#[patch("")]
pub async fn things_patch() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"PATCH: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Delete tihngs")]
#[delete{""}]
pub async fn things_delete() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"DELETE: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
