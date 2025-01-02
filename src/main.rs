use std::{collections::BTreeMap, fs};

use clap::Parser;
use pingora_core::server::Server;
use pingora_core::server::configuration::Opt;
use serde::{Deserialize, Serialize};
use service::{HostConfig, proxy_service};

mod app;
mod service;

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
    servers: BTreeMap<String, HostConfig>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    proxy: Vec<Proxy>,
}

fn main() {
    let opt = Opt::default();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let arg = Args::parse();

    let config_file = fs::read_to_string(arg.config).expect("Failed to open file");
    let config: Config = toml::from_str(&config_file).expect("Failed to deserialize Cargo.toml");



    for i in config.proxy {
        let proxy = proxy_service(&my_server.configuration, &i.listener, i.tls_certificate, i.tls_certificate_key, i.servers);
        my_server.add_service(proxy);
    }

    /*let proxy = proxy_service(
        &my_server.configuration,
        "0.0.0.0:443",
        BTreeMap::from([
            ("admin.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9693".to_owned(),
                proxy_tls: false,
                proxy_hostname: "admin.kargate.site".to_owned(),
                is_websocket: false,
            }),
            ("ssp.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9699".to_owned(),
                proxy_tls: false,
                proxy_hostname: "ssp.kargate.site".to_owned(),
                is_websocket: false,
            }),
            ("landing.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9696".to_owned(),
                proxy_tls: false,
                proxy_hostname: "landing.kargate.site".to_owned(),
                is_websocket: false,
            }),
            ("www.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9696".to_owned(),
                proxy_tls: false,
                proxy_hostname: "www.kargate.site".to_owned(),
                is_websocket: false,
            }),
            ("kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9696".to_owned(),
                proxy_tls: false,
                proxy_hostname: "kargate.site".to_owned(),
                is_websocket: false,
            }),
            ("backend.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9691".to_owned(),
                proxy_tls: false,
                proxy_hostname: "backend.kargate.site".to_owned(),
                is_websocket: false,
            }),
            ("wsb.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9692".to_owned(),
                proxy_tls: false,
                proxy_hostname: "wsb.kargate.site".to_owned(),
                is_websocket: true,
            }),
            ("wsa.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9694".to_owned(),
                proxy_tls: false,
                proxy_hostname: "wsa.kargate.site".to_owned(),
                is_websocket: true,
            }),
            ("wsio.kargate.site".to_owned(), HostConfig {
                proxy_addr: "127.0.0.1:9697".to_owned(),
                proxy_tls: false,
                proxy_hostname: "wsio.kargate.site".to_owned(),
                is_websocket: true,
            }),
        ]),
    );*/

    //my_server.add_service(proxy);
    my_server.run_forever();
}
