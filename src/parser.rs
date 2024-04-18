use clap::{arg, command, error::Result, value_parser, ArgMatches, Error};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub target: String,
    pub method: Method,
    pub proxy: Option<String>,
    pub duration: u64,
    pub connections: usize,
    pub payload: Option<String>,
    pub output: Option<PathBuf>,
    pub headers: Option<HeaderMap>,
    pub timeout: u64,
}

pub struct Parser {
    matches: ArgMatches,
}

impl Parser {
    pub fn new() -> Self {
        let matches = command!()
            .about("Cicadas is a fast multi threaded HTTP load testing tool.")
            .version("1.0.0")
            .arg(
                arg!(
                    -t --target <target> "Target URL for applying load test"
                )
                .required(true)
                .value_parser(value_parser!(String)),
            )
            .arg(
                arg!(
                    -m --method <method> "HTTP method for load testing the target"
                )
                .required(true)
                .value_parser(value_parser!(String)),
            )
            .arg(
                arg!(
                    -P --proxy <proxy> "Optional Proxy URL to use when testing target"
                )
                .required(false)
                .value_parser(value_parser!(String)),
            )
            .arg(
                arg!(
                    -d --duration <duration> "Load test duration in seconds"
                )
                .required(true)
                .value_parser(value_parser!(u64)),
            )
            .arg(
                arg!(
                    -c --connections <connections> "Connections count to open and map with OS threads"
                )
                .required(true)
                .value_parser(value_parser!(usize)),
            )
            .arg(
                arg!(
                    -p --payload <payload> "Optional body payload to pass to the target"
                )
                .required(false)
                .value_parser(value_parser!(String)),
            )
            .arg(
                arg!(
                    -o --output <output> "Optional file path to store target responses"
                )
                .required(false)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -H --headers <headers> "Optional HTTP headers for load testing the target"
                )
                .num_args(0..=255)
                .required(false)
                .value_parser(value_parser!(String)),
            )
            .arg(
                arg!(
                    -T --timeout <timeout> "Optional HTTP request timeout duration in seconds"
                )
                .required(false)
                .default_value("5")
                .value_parser(value_parser!(u64)),
            )
            .get_matches();

        Self { matches }
    }

    pub fn parse_headers(headers: Vec<String>) -> Result<Option<HeaderMap>> {
        let mut headers_map = HeaderMap::with_capacity(headers.len());

        for header in headers {
            let splitted_header: Vec<&str> = header.split(':').collect();

            //TODO: maybe can do better error handling with clap
            if splitted_header.len() != 2 {
                return Err(Error::new(clap::error::ErrorKind::InvalidValue));
            }

            //TODO: handle errors here
            let key: HeaderName = splitted_header[0].parse().unwrap();
            let value: HeaderValue = splitted_header[1].parse().unwrap();

            headers_map.insert(key, value);
        }

        Ok(Some(headers_map))
    }

    pub fn parse_method(method: String) -> Result<Method> {
        let method = method.to_uppercase();

        match method.as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            //TODO: maybe can do better error handling with clap
            _ => Err(Error::new(clap::error::ErrorKind::InvalidValue)),
        }
    }

    pub fn get_arguments(&self) -> Result<Arguments> {
        let target = self.matches.get_one::<String>("target").unwrap().to_owned();
        let raw_method = self.matches.get_one::<String>("method").unwrap().to_owned();
        let method = Self::parse_method(raw_method)?;
        let proxy = match self.matches.get_one::<String>("proxy").to_owned() {
            Some(proxy) => Some(proxy.to_owned()),
            None => None,
        };
        let duration = self.matches.get_one::<u64>("duration").unwrap().to_owned();
        let timeout = self.matches.get_one::<u64>("timeout").unwrap().to_owned();
        let connections = self
            .matches
            .get_one::<usize>("connections")
            .unwrap()
            .to_owned();
        let payload = match self.matches.get_one::<String>("payload").to_owned() {
            Some(payload) => Some(payload.to_owned()),
            None => None,
        };
        let output = match self.matches.get_one::<PathBuf>("output").to_owned() {
            Some(output) => Some(output.to_owned()),
            None => None,
        };
        let raw_headers: Vec<String> = match self.matches.get_many("headers") {
            Some(headers) => {
                let headers: Vec<String> = headers.cloned().collect();

                headers
            }
            None => Vec::with_capacity(0),
        };
        let headers = Self::parse_headers(raw_headers)?;

        Ok(Arguments {
            target,
            method,
            proxy,
            duration,
            connections,
            payload,
            output,
            headers,
            timeout,
        })
    }
}
