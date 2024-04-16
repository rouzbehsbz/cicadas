use reqwest::{blocking::Response, header::HeaderMap, Method, Proxy, Result};

pub struct HttpRequest {
    client: reqwest::blocking::Client,
}

impl HttpRequest {
    pub fn new_blocking(proxy: Option<String>, headers: HeaderMap) -> Result<Self> {
        let mut client_builder = reqwest::blocking::ClientBuilder::new();

        if let Some(proxy_url) = proxy {
            client_builder = client_builder.proxy(Proxy::all(proxy_url)?);
            client_builder = client_builder.default_headers(headers)
        }

        let client = client_builder.build()?;

        Ok(Self { client })
    }

    pub fn call_blocking(
        &self,
        url: &str,
        method: Method,
        payload: Option<String>,
    ) -> Result<Response> {
        let mut request = self.client.request(method, url);

        if let Some(payload) = payload {
            request = request.body(payload);
        }

        let response = request.send()?;

        Ok(response)
    }
}
