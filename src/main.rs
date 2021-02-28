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

    let proxy_config = std::sync::Arc::new(proxy::ReverseProxy {
        client
    });

    let make_svc = make_service_fn(move |_conn| {
        let rp = proxy_config.clone();
        async {
            Ok::<_, proxy::ReverseProxyError>(service_fn(move |req| {
                let rp = rp.clone();
                async move { rp.handle(req).await }
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