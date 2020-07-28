use hyper::{http::uri::InvalidUri, Error as HyperError};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProxyError {
    CacheError,
    ForwardHeaderError,
    InvalidUri(InvalidUri),
    HyperError(HyperError),
    InvalidSocketAddress(String),
}

impl Into<ProxyError> for HyperError {
    fn into(self) -> ProxyError {
        ProxyError::HyperError(self)
    }
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self {
            ProxyError::CacheError => {
                "Error occured while communicating with the cache.".to_string()
            }
            ProxyError::ForwardHeaderError => {
                "Error occurred when forwarding the header to the destination".to_string()
            }
            ProxyError::InvalidUri(uri) => format!("Invalid Uri: {}", uri.to_string()),
            ProxyError::HyperError(err) => format!("Server error occured: {}", err.to_string()),
            ProxyError::InvalidSocketAddress(address) => {
                format!("Invalid address provided {}", address)
            }
        };

        f.write_str(&content)
    }
}

impl Error for ProxyError {}
