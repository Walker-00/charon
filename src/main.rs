use std::{collections::HashMap, fs, time::Duration};

use clap::Parser;
use load_balancer::service::{load_balancer_service, LBHostConfig};
use log::{error, info};
use pingora::prelude::background_service;
use pingora_core::server::Server;
use pingora_core::server::configuration::Opt;
use pingora_load_balancing::{health_check, LoadBalancer};
use proxy::service::{proxy_service, ProxyHostConfig};
use serde::{Deserialize, Serialize};

mod proxy;
mod load_balancer;
mod example_config;

#[derive(clap::Parser, Debug)]
#[command(version, about = "Charon: The Proxy Server", long_about = "Charon is a proxy server, built on Pingora, that ferries packets across the digital river—transferring data from the chaotic internet to servers, much like the mythical Charon guided souls to the underworld.")]
struct Args {
    /// Configuration file path
    #[arg(short, long)]
    config: String,

    /// Get Example Full Config
    #[arg(short = 'e', long, action = clap::ArgAction::SetTrue)]
    example: bool,

    /// Get Example Proxy Config
    #[arg(short = 'p', long, action = clap::ArgAction::SetTrue)]
    example_proxy: bool,

    /// Get Example Load Balancer Config
    #[arg(short = 'l', long, action = clap::ArgAction::SetTrue)]
    example_load_balancer: bool


}

#[derive(Serialize, Deserialize)]
struct ProxyConfig {
    listener: String,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    servers: HashMap<String, ProxyHostConfig>,
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
    servers: HashMap<String, LBHostConfig>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    proxy: Option<Vec<ProxyConfig>>,
    load_balancer: Option<Vec<LoadBalancerConfig>>
}

fn main() {
    env_logger::init();

    let opt = Opt::default();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut proxy_is_configed = false;
    let mut lb_is_configed = false;

    let arg = Args::parse();

    let config_file = fs::read_to_string(arg.config).expect("Failed to open file");
    let config: Config = if let Ok(config) = toml::from_str(&config_file) {
        config
    } else {
        error!("Failed to parse config file");
        info!("Get Example config: charon --example");
        info!("Exiting...");
        std::process::exit(1);
    };

    if let Some(load_balancers) = config.load_balancer {
        if !load_balancers.is_empty() {
            lb_is_configed = true;
        }
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
            my_server.add_service(background);
        }
    }

    if let Some(proxy) = config.proxy {
        if !proxy.is_empty() {
            proxy_is_configed = true; 
        }
        for i in proxy {
            let proxy = proxy_service(&my_server.configuration, &i.listener, i.tls_certificate, i.tls_certificate_key, i.servers);
            my_server.add_service(proxy);
        }
    }

    if !proxy_is_configed || !lb_is_configed {
        error!("No proxy or load balancer is configured!!");
        info!("Get Example config: charon --example");
        info!("Exiting...");
        std::process::exit(1);
    }

    my_server.run_forever();
}
