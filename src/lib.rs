#![doc(html_root_url = "https://marad.github.io/hyper-router/doc/hyper_router")]

//! # Hyper Router
//!
//! This cargo is a small extension to the great Hyper HTTP library. It basically is
//! adds the ability to define routes to request handlers and then query for the handlers
//! by request path.
//!
//! ## Usage
//!
//! To use the library just add: 
//! 
//! ```text
//! hyper-router = "*"
//! ```
//!
//! to your dependencies.
//!
//! ```no_run
//! extern crate hyper;
//! extern crate hyper_router;
//!
//! use hyper::server::{Http, Request, Response};
//! use hyper::header::{ContentLength, ContentType};
//! use hyper_router::{Route, RouterBuilder, RouterService, FutureOr};
//! 
//! fn basic_handler(_: Request) -> FutureOr<Response, hyper::Error> {
//!     let body = "Hello World";
//!     FutureOr::ok_sync(
//!         Response::new()
//!             .with_header(ContentLength(body.len() as u64))
//!             .with_header(ContentType::plaintext())
//!             .with_body(body)
//!     )
//! }
//!
//! fn router_service() -> Result<RouterService, std::io::Error> {
//!     let router = RouterBuilder::new()
//!         .add(Route::get("/greet").using(basic_handler))
//!         .build();
//!     Ok(RouterService::new(router))
//! }
//!
//! fn main() {
//!     let addr = "0.0.0.0:8080".parse().unwrap();
//!     let server = Http::new().bind(&addr, router_service).unwrap();
//!     server.run().unwrap();
//! }
//! ```
//!
//! This code will start Hyper server and add use router to find handlers for request.
//! We create the `Route` so that when we visit path `/greet` the `basic_handler` handler
//! will be called.
//!
//! ## Things to note
//!
//! * `Path::new` method accepts regular expressions so you can match every path you please.
//! * If you have request matching multiple paths the one that was first `add`ed will be chosen.
//! * This library is in an early stage of development so there may be breaking changes comming
//! (but I'll try as hard as I can not to break backwards compatibility or break it just a little - 
//! I promise I'll try!).
//!
//! # Waiting for your feedback
//!
//! I've created this little tool to help myself learn Rust and to avoid using big frameworks
//! like Iron or rustful. I just want to keep things simple.
//!
//! Obviously I could make some errors or bad design choices so I'm waiting for your feedback!
//! You may create an issue at [project's bug tracker](https://github.com/marad/hyper-router/issues).

extern crate futures;
extern crate hyper;

use futures::BoxFuture;
use futures::future::{Either, FutureResult};
use hyper::header::ContentLength;
use hyper::server::{Service, Request, Response};
use hyper::StatusCode;
use hyper::Method;

mod path;
pub mod route;
mod builder;
pub mod handlers;

pub use self::path::Path;
pub use self::route::Route;
pub use self::route::RouteBuilder;
pub use self::builder::RouterBuilder;

pub enum FutureOr<T, E> {
    Sync(Result<T, E>),
    Async(BoxFuture<T, E>)
}

impl<T, E> FutureOr<T, E> {
    pub fn ok_sync(value: T) -> FutureOr<T, E> {
        FutureOr::Sync(Ok(value))
    }
}

pub type Handler = Fn(Request) -> FutureOr<Response, hyper::Error>;
pub type HttpResult<T> = Result<T,StatusCode>;

/// This is the one. The router.
#[derive(Debug)]
pub struct Router {
    routes: Vec<Route>
}

impl Router {
    /// Finds handler for given Hyper request.
    ///
    /// This method uses default error handlers.
    /// If the request does not match any route than default 404 handler is returned.
    /// If the request match some routes but http method does not match (used GET but routes are
    /// defined for POST) than default method not supported handler is returned.
    pub fn find_handler_with_defaults<'a, 'b>(&'a self, request: &'b Request) -> &'a Handler {
        let matching_routes = self.find_matching_routes(request.path());
        match matching_routes.len() {
            x if x <= 0 => &handlers::default_404_handler,
            _ => {
                self.find_for_method(&matching_routes, request.method())
                    .unwrap_or(&handlers::method_not_supported_handler)
            }
        }
    }

    /// Finds handler for given Hyper request.
    ///
    /// It returns handler if it's found or `StatusCode` for error. 
    /// This method may return `NotFound`, `MethodNotAllowed` or `NotImplemented` 
    /// status codes.
    pub fn find_handler<'a, 'b>(&'a self, request: &'b Request) -> HttpResult<&'a Handler> {
        let matching_routes = self.find_matching_routes(request.path());
        match matching_routes.len() {
            x if x <= 0 => Err(StatusCode::NotFound),
            _ => {
                self.find_for_method(&matching_routes, request.method())
                    .map(|handler| Ok(handler))
                    .unwrap_or(Err(StatusCode::MethodNotAllowed))
            }
        }
    }

    /// Returns vector of `Route`s that match to given path.
    pub fn find_matching_routes(&self, request_path: &str) -> Vec<&Route> {
        self.routes.iter()
            .filter(|route| {
                route.path.matcher.is_match(&request_path)
            })
            .collect()
    }

    fn find_for_method<'a, 'b, 'c>(&'a self, routes: &'c Vec<&'a Route>, method: &'b Method) -> Option<&'a Handler> {
        let method = method.clone();
        routes.iter()
            .find(|route| route.method == method)
            .map(|route| &*route.handler)
    }
}

/// The default simple router service.
#[derive(Debug)]
pub struct RouterService {
    pub router: Router,
    pub error_handler: fn(StatusCode) -> FutureOr<Response, hyper::Error>
}

impl RouterService {
    pub fn new(router: Router) -> RouterService {
        RouterService {
            router,
            error_handler: Self::default_error_handler
        }
    }

    fn default_error_handler(status_code: StatusCode) -> FutureOr<Response, hyper::Error> {
        let error = "Routing error: page not found";
        let response = Response::new()
            .with_header(ContentLength(error.len() as u64))
            .with_body(error);

        FutureOr::ok_sync(match status_code {
            StatusCode::NotFound => response.with_status(StatusCode::NotFound),
            _ => response.with_status(StatusCode::InternalServerError)
        })
    }
}

impl Service for RouterService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Either<FutureResult<Response, hyper::Error>, BoxFuture<Response, hyper::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        let future_or = match self.router.find_handler(&request) {
            Ok(handler) => handler(request),
            Err(status_code) => (self.error_handler)(status_code)
        };
        match future_or {
            FutureOr::Sync(res) => Either::A(futures::future::result(res)),
            FutureOr::Async(boxed) => Either::B(boxed)
        }
    }
}
