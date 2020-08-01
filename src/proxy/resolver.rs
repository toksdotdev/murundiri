use super::errors::ProxyError;
use crate::config::{Config, Json, Rule, RuleAction::*, RuleFields, Stringify};
use hyper::{Body, Request, Response, StatusCode};
use hyper_reverse_proxy::{call, ProxyError as HyperReverseProxyError};
use r2d2::{ManageConnection, Pool};
use redis::{transaction, Commands, ConnectionLike};
use std::net::SocketAddr;

pub struct Resolver<M, C>
where
    C: Commands + ConnectionLike + Send + 'static,
    M: ManageConnection<Connection = C>,
{
    socket: SocketAddr,
    config: Config,
    redis: Pool<M>,
}

impl<D, M> Resolver<M, D>
where
    D: Commands + ConnectionLike + Send + 'static,
    M: ManageConnection<Connection = D>,
{
    pub fn new(socket: SocketAddr, config: Config, redis: Pool<M>) -> Self {
        Self {
            socket,
            config,
            redis,
        }
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
        let exists = self.exists_or_create(rule.ttl, &rule.fields).await?;

        match (exists, &rule.action) {
            (false, Redirect { .. }) => Ok(self.forbidden_response(None)),
            (false, Respond { failure, .. }) => Ok(self.forbidden_response(Some(failure))),
            (true, Respond { success, .. }) => Ok(self.ok_response(Some(success))),
            (true, Redirect { uri }) => Ok(call(self.socket.ip(), uri, req).await?),
        }
    }

    async fn exists_or_create(&self, ttl: usize, fields: &RuleFields) -> Result<bool, ProxyError> {
        let key = fields.stringify();
        let mut redis = self.redis.clone().get()?;

        let found = transaction(&mut *redis, &[&key], |conn, pipe| {
            Ok(pipe
                .atomic()
                .getset(&key, true)
                .pexpire(&key, ttl)
                .ignore()
                .query::<Option<(Option<bool>,)>>(conn)?)
        })?;

        Ok(found.0.unwrap_or(false))
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
