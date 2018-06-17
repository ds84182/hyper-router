use Route;
use FutureOr;
use hyper::{Request, Response};
use hyper;

pub struct RouteBuilder {
    route: Route
}

impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder {
            route: route
        }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    pub fn using<T: Fn(Request) -> FutureOr<Response, hyper::Error> + 'static>(mut self, handler: T) -> Route {
        self.route.handler = Box::new(handler);
        self.route
    }
}
