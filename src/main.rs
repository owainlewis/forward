use hyper::service::{make_service_fn, service_fn};

use hyper::{Client, Server};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;

mod proxy;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Construct our HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                proxy::dispatch(req, client.to_owned())
             }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    
    log::info!("Server started on {}", addr);

    if let Err(e) = server.await {
        log::error!("server error: {}", e);
        std::process::abort();
    }
}