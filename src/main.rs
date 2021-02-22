use std::{convert::Infallible, net::SocketAddr};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Method, Request, Response, Server};

type HttpClient = Client<hyper::client::HttpConnector>;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let client = HttpClient::new();

    let make_service = make_service_fn(move |_| {
        let client = client.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| proxy(client.clone(), req))) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn proxy(client: HttpClient, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("req: {:?}", req);
    client.request(req).await
}
