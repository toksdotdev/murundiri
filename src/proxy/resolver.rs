use super::errors::ProxyError;
use crate::config::{Config, Json, Rule, RuleAction::*, RuleFields};
use hyper::{Body, Request, Response, StatusCode};
use hyper_reverse_proxy::{call, ProxyError as HyperReverseProxyError};
use std::{fmt::Debug, net::SocketAddr};

#[derive(Debug)]
pub struct Resolver {
    socket: SocketAddr,
    config: Config,
}

impl Resolver {
    pub fn new(socket: SocketAddr, config: Config) -> Self {
        Self { socket, config }
    }

    pub async fn route(&self, req: Request<Body>) -> Result<Response<Body>, ProxyError> {
        match self.config.get_rule(req.uri().path()) {
            None => Ok(self.internal_server_error_response()),

            Some(rule) => self.handle_rule(rule, req).await,
        }
    }

    async fn handle_rule(
        &self,
        rule: &Rule,
        req: Request<Body>,
    ) -> Result<Response<Body>, ProxyError> {
        let added = self.insert_idempotency(&rule.fields)?;

        match (added, &rule.action) {
            (false, Redirect { .. }) => Ok(self.forbidden_response(None)),
            (false, Respond { failure, .. }) => Ok(self.forbidden_response(Some(failure))),
            (true, Respond { success, .. }) => Ok(self.ok_response(Some(success))),
            (true, Redirect { uri }) => Ok(call(self.socket.ip(), uri, req).await?),
        }
    }

    fn insert_idempotency(&self, fields: &RuleFields) -> Result<bool, ProxyError> {
        // TODO: Insert intor redis here.
        // not found, and inserted => true
        // found, and not inserted => false
        Ok(true)
    }

    /// Get an invalid route response.
    fn ok_response(&self, body: Option<&Json>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .body(body.map_or_else(|| Body::empty(), |j| Body::from(j.0.to_string())))
            .unwrap() // This will always be safe
    }

    fn forbidden_response(&self, body: Option<&Json>) -> Response<Body> {
        Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(body.map_or_else(|| Body::empty(), |j| Body::from(j.0.to_string())))
            .unwrap() // This will always be safe
    }

    fn internal_server_error_response(&self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap() // This will always be safe
    }
}

impl From<HyperReverseProxyError> for ProxyError {
    fn from(err: HyperReverseProxyError) -> Self {
        match err {
            HyperReverseProxyError::InvalidUri(e) => ProxyError::InvalidUri(e),
            HyperReverseProxyError::HyperError(e) => ProxyError::HyperError(e),
            HyperReverseProxyError::ForwardHeaderError => ProxyError::ForwardHeaderError,
        }
    }
}
