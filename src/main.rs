use std::{collections::BTreeMap, fs, time::Duration};

use clap::Parser;
use load_balancer::service::{load_balancer_service, LBHostConfig};
use pingora::prelude::background_service;
use pingora_core::server::Server;
use pingora_core::server::configuration::Opt;
use pingora_load_balancing::{health_check, LoadBalancer};
use proxy::service::{proxy_service, ProxyHostConfig};
use serde::{Deserialize, Serialize};

mod proxy;
mod load_balancer;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long)]
    config: String,
}

#[derive(Serialize, Deserialize)]
struct ProxyConfig {
    listener: String,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    servers: BTreeMap<String, ProxyHostConfig>,
}

#[derive(Serialize, Deserialize)]
struct LoadBalancerConfig {
    listener: String,
    upstreams: Vec<String>,
    health_check: Option<bool>,
    health_check_frequency: Option<u64>,
    parallel_health_check: Option<bool>,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    servers: BTreeMap<String, LBHostConfig>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    proxy: Vec<ProxyConfig>,
    load_balancer: Option<Vec<LoadBalancerConfig>>
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
            let mut upstreams = LoadBalancer::try_from_iter(i.upstreams).unwrap();

            if i.health_check.unwrap_or(false) {
                let hc = health_check::TcpHealthCheck::new();

                upstreams.set_health_check(hc);
                upstreams.parallel_health_check = i.parallel_health_check.unwrap_or(false);

                if let Some(frequency) = i.health_check_frequency {
                    upstreams.health_check_frequency = Some(Duration::from_secs(frequency));
                }

            }

            let background = background_service(&format!("healt check for {}", &i.listener), upstreams);
            let upstreams = background.task();

            let load_balancer = load_balancer_service(&my_server.configuration, &i.listener, i.tls_certificate, i.tls_certificate_key, i.servers, upstreams);
            my_server.add_service(load_balancer);
        }
    }

    for i in config.proxy {
        let proxy = proxy_service(&my_server.configuration, &i.listener, i.tls_certificate, i.tls_certificate_key, i.servers);
        my_server.add_service(proxy);
    }

       my_server.run_forever();
}
