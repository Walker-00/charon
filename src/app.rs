use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use http::header::HOST;
use pingora::{Result, prelude::HttpPeer};
use pingora_proxy::{ProxyHttp, Session};

use crate::service::HostConfig;

pub struct AppProxy {
    pub host_configs: Arc<BTreeMap<String, HostConfig>>,
}

#[async_trait]
impl ProxyHttp for AppProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let host_header = session
            .get_header(HOST)
            .and_then(|h| h.to_str().ok())
            .or_else(|| session.req_header().uri.host())
            .expect("Host header is missing");

        if let Some(host_config) = self.host_configs.get(host_header) {
            let proxy_to = HttpPeer::new(
                &host_config.proxy_addr,
                host_config.proxy_tls,
                host_config.proxy_hostname.clone(),
            );
            Ok(Box::new(proxy_to))
        } else {
            Err(pingora::Error::new(pingora_core::Custom("Host not found")))
        }
    }

    /*async fn upstream_request_filter(
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
            upstream_request
                .insert_header("Upgrade", "websocket")
                .unwrap();
            upstream_request
                .insert_header("Connection", "Upgrade")
                .unwrap();
        }

        Ok(())
    }*/
}
