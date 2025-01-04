use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use http::header::HOST;
use pingora::{Result, prelude::HttpPeer};
use pingora_load_balancing::{prelude::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};

use super::service::LBHostConfig;

pub struct AppLB {
    pub host_configs: Arc<HashMap<String, LBHostConfig>>,
    pub lb_upstreams: Arc<LoadBalancer<RoundRobin>>,
}

#[async_trait]
impl ProxyHttp for AppLB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.lb_upstreams.select(b"", 256).unwrap();

        let host_header = session
            .get_header(HOST)
            .and_then(|h| h.to_str().ok())
            .or_else(|| session.req_header().uri.host())
            .expect("Host header is missing");

        if let Some(host_config) = self.host_configs.get(host_header) {
            let proxy_to = HttpPeer::new(
                upstream,
                host_config.load_balancer_tls,
                host_config.load_balancer_hostname.clone(),
            );
            Ok(Box::new(proxy_to))
        } else {
            Err(pingora::Error::new(pingora_core::Custom("Host not found")))
        }
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let host_header = session
            .get_header(HOST)
            .and_then(|h| h.to_str().ok())
            .or_else(|| session.req_header().uri.host())
            .expect("Host header is missing");
        if let Some(host_config) = self.host_configs.get(host_header) {
            if let Some(headers) = &host_config.load_balancer_headers {
                for (header, value) in headers {
                    upstream_request
                        .insert_header(header.to_owned(), value)
                        .unwrap();
                }
            }
            Ok(())
        } else {
            Err(pingora::Error::new(pingora_core::Custom("Host not found")))
        }
    }
}
