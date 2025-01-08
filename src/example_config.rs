use std::collections::HashMap;

use crate::{load_balancer::service::LBHostConfig, proxy::service::ProxyHostConfig, Config, LoadBalancerConfig, ProxyConfig};

impl Config {
    pub fn new() -> Self {
        Self {
            prometheus_addr: Some("0.0.0.0:9090".to_string()),
            proxy: Some(vec![
                ProxyConfig {
                    listener: "0.0.0.0:8080".to_string(),
                    tls_certificate: None,
                    tls_certificate_key: None,
                    servers: HashMap::from([
                        (
                            "example.com".to_string(),
                            ProxyHostConfig {
                                proxy_addr: "/tmp/example.sock".to_string(),
                                proxy_tls: false,
                                proxy_uds: Some(true),
                                proxy_hostname: "example.com".to_string(),
                                proxy_headers: Some(vec![
                                    ("X-Example-Header".to_string(), "value".to_string())
                                ]),
                            },
                        ),
                        (
                            "another.com".to_string(),
                            ProxyHostConfig {
                                proxy_addr: "127.0.0.1:8001".to_string(),
                                proxy_tls: true,
                                proxy_uds: None,
                                proxy_hostname: "another.com".to_string(),
                                proxy_headers: Some(vec![
                                    ("X-Another-Header".to_string(), "another-value".to_string())
                                ]),
                            },
                        ),
                    ]),
                },
                ProxyConfig {
                    listener: "0.0.0.0:9090".to_string(),
                    tls_certificate: Some("cert.pem".to_string()),
                    tls_certificate_key: Some("key.pem".to_string()),
                    servers: HashMap::from([
                        (
                            "proxyexample.com".to_string(),
                            ProxyHostConfig {
                                proxy_addr: "/tmp/proxy.sock".to_string(),
                                proxy_tls: true,
                                proxy_uds: Some(true),
                                proxy_hostname: "proxyexample.com".to_string(),
                                proxy_headers: Some(vec![
                                    ("X-Proxy-Header".to_string(), "proxy-value".to_string())
                                ]),
                            },
                        ),
                        (
                            "newproxy.com".to_string(),
                            ProxyHostConfig {
                                proxy_addr: "127.0.0.1:9001".to_string(),
                                proxy_tls: false,
                                proxy_uds: None,
                                proxy_hostname: "newproxy.com".to_string(),
                                proxy_headers: Some(vec![
                                    ("X-New-Proxy-Header".to_string(), "new-proxy-value".to_string())
                                ]),
                            },
                        ),
                    ]),
                },
            ]),
            load_balancer: Some(vec![
                LoadBalancerConfig {
                    listener: "0.0.0.0:7070".to_string(),
                    upstreams: vec!["127.0.0.1:7000".to_string(), "127.0.0.1:7001".to_string()],
                    health_check: Some(true),
                    health_check_frequency: Some(30),
                    parallel_health_check: Some(true),
                    tls_certificate: None,
                    tls_certificate_key: None,
                    servers: HashMap::from([
                        (
                            "example.com".to_string(),
                            LBHostConfig {
                                load_balancer_tls: false,
                                load_balancer_hostname: "example.com".to_string(),
                                load_balancer_headers: Some(vec![
                                    ("X-LB-Example".to_string(), "value".to_string())
                                ]),
                            },
                        ),
                        (
                            "another.com".to_string(),
                            LBHostConfig {
                                load_balancer_tls: true,
                                load_balancer_hostname: "another.com".to_string(),
                                load_balancer_headers: Some(vec![
                                    ("X-LB-Another".to_string(), "another-value".to_string())
                                ]),
                            },
                        ),
                    ]),
                },
                LoadBalancerConfig {
                    listener: "0.0.0.0:8080".to_string(),
                    upstreams: vec!["127.0.0.1:8081".to_string(), "127.0.0.1:8082".to_string()],
                    health_check: Some(false),
                    health_check_frequency: None,
                    parallel_health_check: Some(false),
                    tls_certificate: Some("loadbalancer_cert.pem".to_string()),
                    tls_certificate_key: Some("loadbalancer_key.pem".to_string()),
                    servers: HashMap::from([
                        (
                            "proxyexample.com".to_string(),
                            LBHostConfig {
                                load_balancer_tls: true,
                                load_balancer_hostname: "proxyexample.com".to_string(),
                                load_balancer_headers: Some(vec![
                                    ("X-Proxy-LB-Header".to_string(), "proxy-lb-value".to_string())
                                ]),
                            },
                        ),
                        (
                            "newproxy.com".to_string(),
                            LBHostConfig {
                                load_balancer_tls: false,
                                load_balancer_hostname: "newproxy.com".to_string(),
                                load_balancer_headers: Some(vec![
                                    ("X-New-Proxy-LB-Header".to_string(), "new-proxy-lb-value".to_string())
                                ]),
                            },
                        ),
                    ]),
                },
            ]),
        }
    }
    pub fn new_proxy_example() -> Self {
        let mut proxy_servers = HashMap::new();
        proxy_servers.insert("server1".to_string(), ProxyHostConfig {
            proxy_addr: "/tmp/proxy.sock".to_string(),
            proxy_tls: true,
            proxy_uds: Some(true),
            proxy_hostname: "proxy1.com".to_string(),
            proxy_headers: Some(vec![("Header1".to_string(), "Value1".to_string())]),
        });

        let mut proxy_servers2 = HashMap::new();
        proxy_servers2.insert("server2".to_string(), ProxyHostConfig {
            proxy_addr: "192.168.1.2".to_string(),
            proxy_tls: false,
            proxy_uds: None,
            proxy_hostname: "proxy2.com".to_string(),
            proxy_headers: Some(vec![("Header2".to_string(), "Value2".to_string())]),
        });

        Self {
            prometheus_addr: None,
            proxy: Some(vec![
                ProxyConfig {
                    listener: "127.0.0.1:8080".to_string(),
                    tls_certificate: Some("path/to/cert1".to_string()),
                    tls_certificate_key: Some("path/to/key1".to_string()),
                    servers: proxy_servers,
                },
                ProxyConfig {
                    listener: "127.0.0.1:9090".to_string(),
                    tls_certificate: Some("path/to/cert2".to_string()),
                    tls_certificate_key: Some("path/to/key2".to_string()),
                    servers: proxy_servers2,
                },
            ]),
            load_balancer: None,
        }
    }
    pub fn new_load_balancer_example() -> Self {
        let mut lb_servers = HashMap::new();
        lb_servers.insert("lb1".to_string(), LBHostConfig {
            load_balancer_tls: true,
            load_balancer_hostname: "lb1.com".to_string(),
            load_balancer_headers: Some(vec![("Header1".to_string(), "Value1".to_string())]),
        });

        let mut lb_servers2 = HashMap::new();
        lb_servers2.insert("lb2".to_string(), LBHostConfig {
            load_balancer_tls: false,
            load_balancer_hostname: "lb2.com".to_string(),
            load_balancer_headers: Some(vec![("Header2".to_string(), "Value2".to_string())]),
        });

        Self {
            prometheus_addr: Some("0.0.0.0:8080".to_string()),
            proxy: None,
            load_balancer: Some(vec![
                LoadBalancerConfig {
                    listener: "127.0.0.1:8081".to_string(),
                    upstreams: vec!["upstream1".to_string(), "upstream2".to_string()],
                    health_check: Some(true),
                    health_check_frequency: Some(30),
                    parallel_health_check: Some(true),
                    tls_certificate: Some("path/to/lb_cert1".to_string()),
                    tls_certificate_key: Some("path/to/lb_key1".to_string()),
                    servers: lb_servers,
                },
                LoadBalancerConfig {
                    listener: "127.0.0.1:9091".to_string(),
                    upstreams: vec!["upstream3".to_string(), "upstream4".to_string()],
                    health_check: Some(true),
                    health_check_frequency: Some(60),
                    parallel_health_check: Some(false),
                    tls_certificate: Some("path/to/lb_cert2".to_string()),
                    tls_certificate_key: Some("path/to/lb_key2".to_string()),
                    servers: lb_servers2,
                },
            ]),
        }
    }
}
