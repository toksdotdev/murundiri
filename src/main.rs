use clap::{App, Arg};
use dotenv::dotenv;
use murundiri::{
    config::Config,
    proxy::{errors::ProxyError, server::start},
};
use portpicker::pick_unused_port;
use r2d2::Pool;
use redis::{cluster::ClusterClient, Client};
use std::env;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
pub async fn main() -> Result<(), ProxyError> {
    dotenv().ok();

    // Build CLI App
    let matches = App::new("Murundiri")
        .version("1.0")
        .author("Tochukwu N. <nkemdilimtochukwu@gmail.com>")
        .about("Light-weight idempotency reverse-proxy built for scale")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short('c')
                .value_name("FILE")
                .default_value("./config.yml")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ip")
                .long("ip")
                .short('i')
                .value_name("ADDRESS")
                .default_value("0.0.0.0")
                .about("IP Address."),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short('p')
                .default_value("80")
                .about("Run app on port."),
        )
        .get_matches();

    // Get the IP Address.
    let ip = matches.value_of("ip").unwrap();
    let ip: IpAddr = ip
        .parse::<IpAddr>()
        .map_err(|_| ProxyError::InvalidIp(ip.to_string()))?;

    // Get Port
    let port = matches.value_of("port").unwrap();
    let port: u16 = port
        .parse::<u16>()
        .unwrap_or(pick_unused_port().ok_or(ProxyError::NoFreePort)? as u16);

    // Parse Config file
    let config_path = env::current_dir()
        .unwrap()
        .join(matches.value_of("config").unwrap());
    let config = Config::from_file(&config_path)?;

    // Extrac redis Urls
    let redis_urls: Vec<String> = env::var("REDIS_URL")
        .unwrap_or("redis://127.0.0.1:6379".to_string())
        .split(",")
        .map(|e| e.to_string())
        .collect();

    let address: SocketAddr = SocketAddr::new(ip, port);
    println!(
        r#"
        Starting server on: {}
        Config path: {}
        "#,
        address.to_string(),
        &config_path.to_str().unwrap()
    );

    // Setup Redis connection & Start server
    if redis_urls.len() > 1 {
        let client = ClusterClient::open(redis_urls).unwrap();
        let pool = Pool::builder().build(client).unwrap();
        return start(address, Config::default(), pool.clone()).await;
    }

    let client = Client::open(redis_urls[0].to_owned()).unwrap();
    let pool = Pool::builder().build(client).unwrap();
    return start(address, config, pool.clone()).await;
}
