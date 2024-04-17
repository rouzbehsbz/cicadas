use std::time::Duration;

use reqwest::{blocking::Response, Method, Proxy, Result, StatusCode};

use crate::parser::Arguments;

#[derive(PartialEq, Eq, Hash)]
pub enum StatusCodeCategory {
    Success,
    Redirection,
    ClientError,
    ServerError,
    Informational,
    Failed,
}

impl From<StatusCode> for StatusCodeCategory {
    fn from(value: StatusCode) -> Self {
        if value.is_success() {
            Self::Success
        } else if value.is_redirection() {
            Self::Redirection
        } else if value.is_client_error() {
            Self::ClientError
        } else if value.is_server_error() {
            Self::ServerError
        } else {
            Self::Informational
        }
    }
}

#[derive(Clone)]
pub struct HttpClientBlocking {
    client: reqwest::blocking::Client,
    url: String,
    method: Method,
    payload: Option<String>,
}

impl HttpClientBlocking {
    pub fn from_arguments(arguments: &Arguments) -> Result<Self> {
        let mut client_builder = reqwest::blocking::ClientBuilder::new();

        if let Some(proxy_url) = &arguments.proxy {
            client_builder = client_builder.proxy(Proxy::all(proxy_url)?);
        }
        if let Some(headers) = &arguments.headers {
            client_builder = client_builder.default_headers(headers.clone());
        }

        client_builder = client_builder.timeout(Duration::from_secs(arguments.timeout));
        client_builder = client_builder.tcp_keepalive(Duration::from_secs(arguments.duration));

        let client = client_builder.build()?;

        Ok(Self {
            client,
            url: arguments.target.clone(),
            method: arguments.method.clone(),
            payload: arguments.payload.clone(),
        })
    }

    pub fn call(&self) -> Result<Response> {
        let mut request = self.client.request(self.method.clone(), self.url.clone());

        if let Some(payload) = &self.payload {
            request = request.body(payload.clone());
        }

        let response = request.send()?;

        Ok(response)
    }
}
