use crate::config::ConfigParseError;
use hyper::{http::uri::InvalidUri, Error as HyperError};
use r2d2::Error as R2d2Error;
use redis::RedisError;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProxyError {
    ForwardHeaderError,
    NoFreePort,
    InvalidIp(String),
    CacheError(RedisError),
    InvalidUri(InvalidUri),
    HyperError(HyperError),
    InvalidSocketAddress(String),
    ConfigParseError(ConfigParseError),
    RedisConnectionPoolError(R2d2Error),
}

impl From<HyperError> for ProxyError {
    fn from(err: HyperError) -> Self {
        ProxyError::HyperError(err)
    }
}

impl From<RedisError> for ProxyError {
    fn from(err: RedisError) -> Self {
        ProxyError::CacheError(err)
    }
}

impl From<ConfigParseError> for ProxyError {
    fn from(err: ConfigParseError) -> Self {
        ProxyError::ConfigParseError(err)
    }
}

impl From<R2d2Error> for ProxyError {
    fn from(err: R2d2Error) -> Self {
        ProxyError::RedisConnectionPoolError(err)
    }
}

impl Error for ProxyError {}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self {
            ProxyError::ConfigParseError(err) => err.to_string(),
            ProxyError::InvalidIp(ip) => format!("Invalid IP Address: {}", ip),
            ProxyError::RedisConnectionPoolError(err) => {
                format!("Redis error: {}", err.to_string())
            }
            ProxyError::CacheError(err) => format!("Redis error: {}", err.to_string()),
            ProxyError::ForwardHeaderError => {
                "Error occurred when forwarding the header to the destination".to_string()
            }
            ProxyError::InvalidUri(uri) => format!("Invalid Uri: {}", uri.to_string()),
            ProxyError::HyperError(err) => format!("Server error occured: {}", err.to_string()),
            ProxyError::InvalidSocketAddress(address) => {
                format!("Invalid address provided {}", address)
            }
            ProxyError::NoFreePort => "All system ports are exhausted".to_string(),
        };

        f.write_str(&content)
    }
}
