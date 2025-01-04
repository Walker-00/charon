use std::{collections::BTreeMap, sync::Arc};

use pingora::{server::configuration::ServerConf, services::listening::Service};
use pingora_proxy::HttpProxy;
use serde::{Deserialize, Serialize};

use super::app::AppProxy;

#[derive(Serialize, Deserialize)]
pub struct ProxyHostConfig {
    pub proxy_addr: String,
    pub proxy_tls: bool,
    pub proxy_hostname: String,
    pub proxy_headers: Option<Vec<(String, String)>>,
}

pub fn proxy_service(
    server_conf: &Arc<ServerConf>,
    listen_addr: &str,
    tls_certificate: Option<String>,
    tls_certificate_key: Option<String>,
    host_configs: BTreeMap<String, ProxyHostConfig>,
) -> Service<HttpProxy<AppProxy>> {
    let host_configs = Arc::new(host_configs);
    let mut proxy = pingora_proxy::http_proxy_service(server_conf, AppProxy { host_configs });
    if let (Some(cert_path), Some(key_path)) = (tls_certificate, tls_certificate_key) {
        let mut tls_settings =
            pingora_core::listeners::tls::TlsSettings::intermediate(&cert_path, &key_path).expect("TLS error");
        tls_settings.enable_h2();
        proxy.add_tls_with_settings(listen_addr, None, tls_settings);    
        return proxy;
    }

    println!("TLS disabled for {}", listen_addr);

    proxy.add_tcp(listen_addr);
    proxy
}
