use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use http::header::HOST;
use pingora::{Result, prelude::HttpPeer};
use pingora_proxy::{ProxyHttp, Session};

use crate::structures::proxy_structure::ProxyHostConfig;

pub struct AppProxy {
    pub host_configs: Arc<HashMap<String, ProxyHostConfig>>,
}

#[async_trait]
impl ProxyHttp for AppProxy {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let request_path = session.req_header().uri.path();
        let host_header = session
            .get_header(HOST)
            .and_then(|h| h.to_str().ok())
            .or_else(|| session.req_header().uri.host())
            .expect("Host header is missing");

        if let Some(host_config) = self.host_configs.get(host_header) {
            let mut proxy_uds = host_config.proxy_uds;
            let mut proxy_addr = &host_config.proxy_addr;
            let mut proxy_tls = host_config.proxy_tls;
            if let Some(routes) = &host_config.routes {
                if let Some(route_host_config) = routes.get(request_path) {
                    println!("{route_host_config:?}");
                    println!("bruh");
                    if let Some(uds) = route_host_config.proxy_uds {
                        proxy_uds = Some(uds);
                    }

                    if let Some(addr) = &route_host_config.proxy_addr {
                        println!("{addr}");
                        proxy_addr = &addr;
                    }

                    if let Some(tls) = route_host_config.proxy_tls {
                        proxy_tls = tls;
                    }
                }
            }
            if let Some(true) = proxy_uds {
                return Ok(Box::new(
                    HttpPeer::new_uds(&proxy_addr, proxy_tls, host_header.to_string()).unwrap(),
                ));
            }
            let proxy_to = HttpPeer::new(&proxy_addr, proxy_tls, host_header.to_string());
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
        let request_path = session.req_header().uri.path();
        let host_header = session
            .get_header(HOST)
            .and_then(|h| h.to_str().ok())
            .or_else(|| session.req_header().uri.host())
            .expect("Host header is missing");
        if let Some(host_config) = self.host_configs.get(host_header) {
            let mut proxy_headers = &host_config.proxy_headers;
            if let Some(routes) = &host_config.routes {
                if let Some(route_host_config) = routes.get(request_path) {
                    if route_host_config.proxy_headers.is_some() {
                        proxy_headers = &route_host_config.proxy_headers;
                    }
                }
            }
            if let Some(headers) = proxy_headers {
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
