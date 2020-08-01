pub mod errors;
pub mod resolver;
pub mod server;

// Async transactions aren't currently supported.
// See link for workaround https://github.com/mitsuhiko/redis-rs/issues/353#issuecomment-666290557
#[macro_export]
macro_rules! async_transaction {
    ($conn:expr, $keys:expr, $body:expr) => {
        loop {
            redis::cmd("WATCH").arg($keys).query_async($conn).await?;

            if let Some(response) = $body {
                redis::cmd("UNWATCH").query_async($conn).await?;
                break response;
            }
        }
    };
}
