use super::resolver::Resolver;
use crate::{config::Config, proxy::errors::ProxyError};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use r2d2::{ManageConnection, Pool};
use redis::{AsyncCommands, ConnectionLike};
use std::{convert::Infallible, net::SocketAddr, sync::Arc};

pub async fn start<C, M>(
    address: SocketAddr,
    config: Config,
    redis_pool: Pool<M>,
) -> Result<(), ProxyError>
where
    C: AsyncCommands + Copy + Send + 'static,
    M: ConnectionLike + ManageConnection<Connection = C>,
{
    let resolver = Arc::new(Resolver::new(address, config, redis_pool));

    let make_svc = make_service_fn(move |_| {
        let temp_resolver = Arc::clone(&resolver);

        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let e = Arc::clone(&temp_resolver);

                async move { e.route(req).await }
            }))
        }
    });

    Server::bind(&address)
        .serve(make_svc)
        .await
        .map_err(|e| e.into())
}
