use hyper::server::{Request, Response};
use hyper::StatusCode;
use hyper::header::{ContentLength, ContentType};
use hyper;
use super::FutureOr;

pub fn default_404_handler(_: Request) -> FutureOr<Response, hyper::Error> {
    let body = "page not found";
    FutureOr::ok_sync(
        Response::new()
            .with_status(StatusCode::NotFound)
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(body)
    )
}

pub fn method_not_supported_handler(_: Request) -> FutureOr<Response, hyper::Error> {
    let body = "method not supported";
    FutureOr::ok_sync(
    Response::new()
            .with_status(StatusCode::MethodNotAllowed)
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(body)
    )
}

pub fn internal_server_error_handler(_: Request) -> FutureOr<Response, hyper::Error> {
    let body = "internal server error";
    FutureOr::ok_sync(
    Response::new()
            .with_status(StatusCode::InternalServerError)
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(body)
    )
}

pub fn not_implemented_handler(_: Request) -> FutureOr<Response, hyper::Error> {
    let body = "not implemented";
    FutureOr::ok_sync(
    Response::new()
            .with_status(StatusCode::NotImplemented)
            .with_header(ContentLength(body.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(body)
    )
}
