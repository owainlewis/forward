use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Server, Request, Response, Body, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;

type HttpClient    = Client<HttpsConnector<hyper::client::HttpConnector>>;
type ProxyResponse = Result<Response<Body>, hyper::Error>;

// Proxy a request to a backend server
async fn proxy_request(mut req: Request<Body>, client: HttpClient) -> ProxyResponse {
    // Modify request here 
    let uri = "https://github.com/".parse::<Uri>().unwrap();
    *req.uri_mut() = uri;

    // Dispatch
    let response = match client.request(req).await {
        Ok(res) => res,
        Err(e) => {
            return Ok(
                Response::builder().
                status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Could not complete request: \"{}\"", e).into())
                .unwrap()
            )
        }
    };

    // Modify repsonse here

    Ok(response)    
}

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
                proxy_request(req, client.to_owned())
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