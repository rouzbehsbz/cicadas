use clap::error::ErrorKind;

pub type AppResult<T> = Result<T, ErrorType>;

pub enum ErrorType {
    InvalidHeaderStructure,
    InvalidHeaderName,
    InvalidHeaderValue,
    InvalidMethod,
    InvalidProxy,
    HttpClientBuildFailed,
    HttpRequestFailed,
    InvalidError,
}

impl ErrorType {
    pub fn to_command_error(&self) -> (ErrorKind, &str) {
        match *self {
            ErrorType::InvalidHeaderStructure => {
                (ErrorKind::InvalidValue, "Invalid header structure. Headers must be provided in the following format: <HEADER_NAME>:<HEADER_VALUE>")
            }
            ErrorType::InvalidHeaderName => (ErrorKind::InvalidValue, "Invalid header name. Can't parse header name."),
            ErrorType::InvalidHeaderValue => (ErrorKind::InvalidValue, "Invalid header value. Can't parse header value."),
            ErrorType::InvalidMethod => (ErrorKind::InvalidValue, "Invalid method. Please use one of the GET, POST, PUT, DELETE, HEAD and OPTIONS methods."),
            ErrorType::InvalidProxy => (ErrorKind::InvalidValue, "Invalid proxy. Please provide valid http, https or socks proxy."),
            ErrorType::HttpClientBuildFailed => (ErrorKind::Io, "Failed to build HTTP client. TLS backend can't initialized or system configuration can't load probably."),
            ErrorType::HttpRequestFailed => (ErrorKind::Io, "HTTP request failed due to timeout reached."),
            ErrorType::InvalidError => (ErrorKind::InvalidValue, "Invalid error. Please report this error to our GitHub issues page."),
        }
    }
}
