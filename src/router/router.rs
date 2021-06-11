 pub struct Route {
    /// HTTP method to match
    pub method: Method,
    /// Path to match
    pub path: Path,
    /// Request handler
    ///
    /// This should be method that accepts Hyper's Request and Response:
    ///
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper::header::{ContentLength, ContentType};
    ///
    /// fn hello_handler(_: Request) -> Response {
    ///     let body = "Hello World";
    ///     Response::new()
    ///         .with_header(ContentLength(body.len() as u64))
    ///         .with_header(ContentType::plaintext())
    ///         .with_body(body)
    /// }
    /// ```
    pub handler: Handler,
}