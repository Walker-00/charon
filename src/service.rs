use std::sync::Arc;

use hashbrown::HashMap;
use pingora::{server::configuration::ServerConf, services::listening::Service};
use pingora_proxy::HttpProxy;
use serde::Serialize;

use crate::app::AppProxy;

#[derive(Serialize)]
pub struct HostConfig {
    pub proxy_addr: String,
    pub proxy_tls: bool,
    pub proxy_hostname: String,
    pub is_websocket: bool,
}

pub fn proxy_service(
    server_conf: &Arc<ServerConf>,
    listen_addr: &str,
    host_configs: HashMap<String, HostConfig>,
) -> Service<HttpProxy<AppProxy>> {
    let host_configs = Arc::new(host_configs);
    let mut proxy = pingora_proxy::http_proxy_service(server_conf, AppProxy { host_configs });
    let cert_path = "/etc/letsencrypt/live/kargate.site/fullchain.pem";
    let key_path = "/etc/letsencrypt/live/kargate.site/privkey.pem";

    let mut tls_settings =
        pingora_core::listeners::tls::TlsSettings::intermediate(cert_path, key_path).expect("TLS error");
    tls_settings.enable_h2();
    proxy.add_tls_with_settings(listen_addr, None, tls_settings);

    proxy
}
