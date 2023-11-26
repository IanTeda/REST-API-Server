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
use actix_web::{web, Error, HttpResponse};
use futures::{future::ok, stream::once};

#[tracing::instrument(name = "Get tihngs")]
pub async fn get_things() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"GET: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Put tihngs")]
pub async fn post_things() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"POST: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Put tihngs")]
pub async fn put_things() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"PUT: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Patch tihngs")]
pub async fn patch_things() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"PATCH: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[tracing::instrument(name = "Delete tihngs")]
pub async fn delete_things() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"DELETE: Things")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
