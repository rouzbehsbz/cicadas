use clap::{arg, command, error::Result, value_parser, ArgMatches, Error, ValueEnum};
use std::path::PathBuf;

pub type Headers = Vec<[String; 2]>;

#[derive(Debug, Clone, ValueEnum)]
pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    OPTIONS,
    HEAD,
}

#[derive(Debug)]
pub struct Arguments {
    target: String,
    method: Method,
    proxy: Option<String>,
    duration: u32,
    payload: Option<String>,
    output: Option<PathBuf>,
    headers: Headers,
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
                .value_parser(value_parser!(u32)),
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
            .get_matches();

        Self { matches }
    }

    pub fn parse_headers(headers: Vec<String>) -> Result<Headers> {
        let mut parsed_headers = Vec::with_capacity(headers.len());

        for header in headers {
            let splitted_header: Vec<&str> = header.split(':').collect();

            //TODO: maybe can do better error handling with clap
            if splitted_header.len() != 2 {
                return Err(Error::new(clap::error::ErrorKind::InvalidValue));
            }

            parsed_headers.push([
                splitted_header[0].to_string(),
                splitted_header[1].to_string(),
            ])
        }

        Ok(parsed_headers)
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
        let duration = self.matches.get_one::<u32>("duration").unwrap().to_owned();
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
            payload,
            output,
            headers,
        })
    }
}
