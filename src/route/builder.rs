use Route;
use Handler;
use hyper::{Request, Response};

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
    pub fn using<T: Fn(Request) -> Response + 'static>(mut self, handler: T) -> Route {
        self.route.handler = Box::new(handler);
        self.route
    }
}
