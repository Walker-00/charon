use std::{collections::BTreeMap, fs};

use clap::Parser;
use pingora_core::server::Server;
use pingora_core::server::configuration::Opt;
use proxy::service::{proxy_service, ProxyHostConfig};
use serde::{Deserialize, Serialize};

mod proxy;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long)]
    config: String,
}

#[derive(Serialize, Deserialize)]
struct Proxy {
    listener: String,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    servers: BTreeMap<String, ProxyHostConfig>,
}

#[derive(Serialize, Deserialize)]
struct LoadBalancer {
    listener: String,
    health_check: Option<bool>,
    health_check_frequency: Option<u32>,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    servers: BTreeMap<String, ProxyHostConfig>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    proxy: Vec<Proxy>,
    load_balancer: Option<Vec<LoadBalancer>>
}

fn main() {
    let opt = Opt::default();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let arg = Args::parse();

    let config_file = fs::read_to_string(arg.config).expect("Failed to open file");
    let config: Config = toml::from_str(&config_file).expect("Failed to deserialize Cargo.toml");

    if let Some(load_balancers) = config.load_balancer {
        for i in load_balancers {
            let load_balancer = proxy_service(&my_server.configuration, &i.listener, i.tls_certificate, i.tls_certificate_key, i.servers);
            my_server.add_service(load_balancer);
        }
    }

    for i in config.proxy {
        let proxy = proxy_service(&my_server.configuration, &i.listener, i.tls_certificate, i.tls_certificate_key, i.servers);
        my_server.add_service(proxy);
    }

       my_server.run_forever();
}
