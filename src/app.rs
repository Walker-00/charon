use async_trait::async_trait;
use hashbrown::HashMap;
use http::header::HOST;
use pingora::{Result, prelude::HttpPeer};
use pingora_proxy::{ProxyHttp, Session};

use crate::service::HostConfig;

pub struct AppProxy {
    pub host_configs: HashMap<String, HostConfig>,
}

#[async_trait]
impl ProxyHttp for AppProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let host_header = if let Some(host_header) = session.get_header(HOST) {
            host_header.to_str().unwrap()
        } else {
            session.req_header().uri.host().unwrap()
        };

        let host_config = self.host_configs.get(host_header).unwrap();

        let proxy_to = HttpPeer::new(
            host_config.proxy_addr.as_str(),
            host_config.proxy_tls,
            host_config.proxy_hostname.clone(),
        );

        let peer = Box::new(proxy_to);
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let host_header = if let Some(host_header) = session.get_header(HOST) {
            host_header.to_str().unwrap()
        } else {
            session.req_header().uri.host().unwrap()
        };

        let host_config = self.host_configs.get(host_header).unwrap();

        /*let host_config = self
            .host_configs
            .iter()
            .find(|x| x.proxy_hostname == host_header)
            .unwrap();*/

        if host_config.is_websocket {
            upstream_request.insert_header("Upgrade", "websocket").unwrap();
            upstream_request.insert_header("Connection", "Upgrade").unwrap();
        }

        Ok(())
    }
}
