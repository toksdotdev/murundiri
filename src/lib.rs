pub mod config;
pub mod proxy;
pub mod utils;

#[macro_export]
macro_rules! hashmap_populate {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}
// let address: SocketAddr = bind_address
//         .parse()
//         .map_err(|err: AddrParseError| ProxyError::InvalidSocketAddress(err.to_string()))?;
