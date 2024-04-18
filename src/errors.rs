use thiserror::Error;

pub type AppResult<T> = Result<T, ErrorType>;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("Invalid header structure.")]
    InvalidHeaderStructure,
    #[error("Cannot parse one of the header names.")]
    InvalidHeaderName,
    #[error("Cannot parse one of the header values.")]
    InvalidHeaderValue,
    #[error("Invalid HTTP method.")]
    InvalidMethod,
    #[error("Invalid Proxy.")]
    InvalidProxy,
    #[error("Failed to initialize HTTP Client.")]
    HttpClientBuildFailed,
    #[error("Failed to receive HTTP response.")]
    HttpRequestFailed,
    #[error("Failed to execute a thread.")]
    ThreadJoinFailed,
    #[error("Unexpected error. this should not happen.")]
    InvalidError,
}
