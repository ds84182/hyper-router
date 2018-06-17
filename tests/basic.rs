extern crate hyper;
extern crate hyper_router;

use hyper_router::*;
use hyper::server::{Request, Response};
use hyper::Method;
use hyper::Uri;
use std::str::FromStr;


#[test]
fn test_get_route() {
    let request = Request::new(Method::Get, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_get_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_get_root(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };
    fn handle_get_foo(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::get("/hello").using(handle_get_hello))
        .add(Route::get("/").using(handle_get_root))
        .add(Route::get("/foo").using(handle_get_foo))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_post_route() {
    let request = Request::new(Method::Post, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_root(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };
    fn handle_post_foo(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };
    fn handle_get_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::post("/hello").using(handle_post_hello))
        .add(Route::get("/").using(handle_post_root))
        .add(Route::get("/foo").using(handle_post_foo))
        .add(Route::get("/hello").using(handle_get_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_delete_route() {
    let request = Request::new(Method::Delete, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_delete_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::delete("/hello").using(handle_delete_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_options_route() {
    let request = Request::new(Method::Options, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_options_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::options("/hello").using(handle_options_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_put_route() {
    let request = Request::new(Method::Put, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_put_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::put("/hello").using(handle_put_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_head_route() {
    let request = Request::new(Method::Head, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_head_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::head("/hello").using(handle_head_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_trace_route() {
    let request = Request::new(Method::Trace, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_trace_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::trace("/hello").using(handle_trace_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_patch_route() {
    let request = Request::new(Method::Patch, Uri::from_str("http://www.example.com/hello").unwrap());

    fn handle_patch_hello(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_post_hello(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::patch("/hello").using(handle_patch_hello))
        .add(Route::post("/hello").using(handle_post_hello))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}


#[test]
fn test_no_route() {
    let request = Request::new(Method::Get, Uri::from_str("http://www.example.com/notfound").unwrap());

    fn handle_get_foo(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };
    fn handle_get_bar(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::patch("/foo").using(handle_get_foo))
        .add(Route::patch("/bar").using(handle_get_bar))
        .build();

    let handler = router.find_handler(&request);

    match handler {
        Ok(_) => panic!("Expected an error, but got a handler instead"),
        Err(e) => assert_eq!(e, hyper::StatusCode::NotFound)
    }
}


#[test]
fn test_regex_path() {
    let request = Request::new(Method::Get, Uri::from_str("http://www.example.com/foo/bar").unwrap());

    fn handle_regex_foo(_: Request) -> FutureOr<Response, hyper::Error> { FutureOr::ok_sync(Response::new()) };
    fn handle_regex_bar(_: Request) -> FutureOr<Response, hyper::Error> { unimplemented!() };

    let router = RouterBuilder::new()
        .add(Route::get(r"/foo/.*?").using(handle_regex_foo))
        .add(Route::get(r"/bar/.*?").using(handle_regex_bar))
        .build();

    let handler = router.find_handler(&request).unwrap();
    handler(request);
}
