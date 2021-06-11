use hyper::{HeaderMap};
use hyper::{Client, Request, Response, Body, StatusCode, Uri};
use hyper_tls::HttpsConnector;

type HttpClient    = Client<HttpsConnector<hyper::client::HttpConnector>>;
type ProxyResponse = Result<Response<Body>, hyper::Error>;

#[derive(Clone, Debug)]
pub struct Proxy<T> { 
  client: hyper::Client<T>
}

fn not_found_handler(_: Request<Body>) -> Response<Body> {    
    return Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(Body::from("Not Found"))
    .unwrap();
}

fn error_handler(e: hyper::Error) -> Response<Body> {
    return Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Could not complete request: \"{}\"", e).into())
        .unwrap();
}

// Remove hop headers from request
fn remove_hop_headers(headers: &mut HeaderMap) {
    let hop = vec![
        "Host",
        "Connection", 
        "Keep-Alive",
        "Proxy-Authenticate",
        "Proxy-Authorization",
        "Te",
        "Trailers",
        "Transfer-Encoding",
        "Upgrade"
    ];    

    for (k,_) in headers.clone().iter() {        
        if hop.iter().any(|h| h == k) {
            headers.remove(k);
        }
    }
}

// Proxy a request to a backend server
pub async fn dispatch(mut req: Request<Body>, client: HttpClient) -> ProxyResponse {
    // Modify request here 
    
    if req.uri().path().contains("/foo") {
        *req.uri_mut() = "https://jsonplaceholder.typicode.com/todos/1".parse::<Uri>().unwrap();
    } else {
        *req.uri_mut() = "https://jsonplaceholder.typicode.com/todos/2".parse::<Uri>().unwrap();
    }

    remove_hop_headers(req.headers_mut());

    // Dispatch
    let response = match client.request(req).await {
        Ok(res) => res,
        Err(e) => return Ok(error_handler(e))
    };

    // Modify repsonse here    
    Ok(response)    
}