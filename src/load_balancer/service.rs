use std::{collections::HashMap, sync::Arc};

use pingora::{server::configuration::ServerConf, services::listening::Service};
use pingora_load_balancing::{prelude::RoundRobin, LoadBalancer};
use pingora_proxy::HttpProxy;
use serde::{Deserialize, Serialize};

use super::app::AppLB;

#[derive(Serialize, Deserialize)]
pub struct LBHostConfig {
    pub load_balancer_tls: bool,
    pub load_balancer_hostname: String,
    pub load_balancer_headers: Option<Vec<(String, String)>>,
}

pub fn load_balancer_service(
    server_conf: &Arc<ServerConf>,
    listen_addr: &str,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    host_configs: HashMap<String, LBHostConfig>,
    lb_upstreams: Arc<LoadBalancer<RoundRobin>>
) -> Service<HttpProxy<AppLB>> {
    let host_configs = Arc::new(host_configs);
    let mut lb = pingora_proxy::http_proxy_service(server_conf, AppLB { host_configs,  lb_upstreams });
    if let (Some(cert_path), Some(key_path)) = (tls_certificate, tls_certificate_key) {
        let mut tls_settings =
            pingora_core::listeners::tls::TlsSettings::intermediate(&cert_path, &key_path).expect("TLS error");
        tls_settings.enable_h2();
        lb.add_tls_with_settings(listen_addr, None, tls_settings);    
        return lb;
    }

    println!("TLS disabled for {}", listen_addr);

    lb.add_tcp(listen_addr);
    lb
}
