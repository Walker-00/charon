use async_trait::async_trait;
use http::header;

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_proxy::{ProxyHttp, Session};

pub struct HostConfig {
    pub proxy_addr: String,
    pub proxy_tls: bool,
    pub proxy_hostname: String
}

pub struct LB {
    host_configs: Vec<HostConfig>,
}

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let host_header = session.get_header(header::HOST).unwrap().to_str().unwrap();
        println!("{host_header}");

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

// RUST_LOG=INFO cargo run --example load_balancer
fn main() {
    let opt = Opt::parse_args();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut lb = pingora_proxy::http_proxy_service(&my_server.configuration, LB {
        host_configs: vec![
            HostConfig {
                proxy_addr: "127.0.0.1:9691".to_owned(),
                proxy_tls: false,
                proxy_hostname: "backend.kargate.site".to_owned()
            }
        ]
    });
    lb.add_tcp("0.0.0.0:443");

    my_server.add_service(lb);
    my_server.run_forever();
}
