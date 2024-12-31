use pingora::prelude::{Result, Session, ProxyHttp, HttpPeer};

mod service;
mod app;

#[derive(Clone)]
struct HostConfig {
    proxy_addr: String,
    proxy_hostname: String,
}

struct ProxyApp {
    host_configs: Vec<HostConfig>
}

impl ProxyApp {
    fn new(host_configs: Vec<HostConfig>) -> Self {
        Self {
            host_configs
        }
    }
}

impl ProxyHttp for ProxyApp {
    type CTX = ();
    fn new_ctx(&self) {
    }

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let host_header = session.get_header(header::HOST).unwrap().to_str().unwrap();

        let host_config = self
            .host_configs
            .iter()
            .find(|x| x.proxy_hostname == host_header)
            .unwrap();
        let proxy_to = HttpPeer::new(
            host_config.proxy_addr.as_str(),
            host_config.proxy_tls,
            host_config.proxy_hostname.clone(),
        );
        let peer = Box::new(proxy_to);
        Ok(peer)
    }
}

fn main() {
    println!("Hello, world!");
}
