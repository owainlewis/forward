use hyper::{Body, Client, Request, Response};
use hyper_tls::HttpsConnector;

#[derive(Debug)]
pub enum ReverseProxyError {
    Hyper(hyper::Error),
    HyperHttp(hyper::http::Error),
}

impl From<hyper::Error> for ReverseProxyError {
    fn from(e: hyper::Error) -> Self {
        ReverseProxyError::Hyper(e)
    }
}

impl From<hyper::http::Error> for ReverseProxyError {
    fn from(e: hyper::http::Error) -> Self {
        ReverseProxyError::HyperHttp(e)
    }
}

impl std::fmt::Display for ReverseProxyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for ReverseProxyError {}

pub struct ReverseProxy {
    pub client: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl ReverseProxy {    
    // Remove proxy hop headers
    fn strip_proxy_headers(&self, mut req: Request<Body>) -> Request<Body> {
        const STRIPPED: [&str; 6] = [
            "content-length",
            "transfer-encoding",
            "accept-encoding",
            "content-encoding",
            "host",
            "connection",
        ];
        let h = req.headers_mut();
        for key in &STRIPPED {
            h.remove(*key);
        }

        req
    }

    pub async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, ReverseProxyError> {
        let mut updated = self.strip_proxy_headers(req);

        let mut builder = hyper::Uri::builder()
            .scheme("https")
            .authority("httpbin.org");
             
        // Todo request modifications like copying headers etc 
        if let Some(pq) = updated.uri().path_and_query() {
            builder = builder.path_and_query(pq.clone());
        }

        *updated.uri_mut() = builder.build()?;

        log::info!("request == {:?}", updated);
        let response = self.client.request(updated).await?;
        log::debug!("response == {:?}", response);
        Ok(response)
    }
}