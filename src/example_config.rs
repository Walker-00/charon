use std::collections::HashMap;

use crate::{
    Config, LoadBalancerConfig, ProxyConfig,
    structures::{
        load_balancer_structure::LBHostConfig,
        proxy_structure::{ProxyHostConfig, ProxyPathBaseHostConfig},
    },
};

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
                        ("example.com".to_string(), ProxyHostConfig {
                            proxy_addr: "/tmp/example.sock".to_string(),
                            proxy_tls: false,
                            proxy_uds: Some(true),
                            proxy_headers: Some(vec![(
                                "X-Example-Header".to_string(),
                                "value".to_string(),
                            )]),
                            routes: Some(HashMap::from([
                                ("/api/v1".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: Some("/tmp/example.sock".to_string()),
                                    proxy_tls: Some(false),
                                    proxy_uds: Some(true),
                                    proxy_headers: None,
                                }),
                                ("/status".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(false),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                            ])),
                        }),
                        ("another.com".to_string(), ProxyHostConfig {
                            proxy_addr: "127.0.0.1:8001".to_string(),
                            proxy_tls: true,
                            proxy_uds: None,
                            proxy_headers: Some(vec![(
                                "X-Another-Header".to_string(),
                                "another-value".to_string(),
                            )]),
                            routes: Some(HashMap::from([
                                ("/home".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(true),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                                ("/login".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(true),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                            ])),
                        }),
                    ]),
                },
                ProxyConfig {
                    listener: "0.0.0.0:9090".to_string(),
                    tls_certificate: Some("cert.pem".to_string()),
                    tls_certificate_key: Some("key.pem".to_string()),
                    servers: HashMap::from([
                        ("proxyexample.com".to_string(), ProxyHostConfig {
                            proxy_addr: "/tmp/proxy.sock".to_string(),
                            proxy_tls: true,
                            proxy_uds: Some(true),
                            proxy_headers: Some(vec![(
                                "X-Proxy-Header".to_string(),
                                "proxy-value".to_string(),
                            )]),
                            routes: Some(HashMap::from([
                                ("/data".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(true),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                                ("/info".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(true),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                            ])),
                        }),
                        ("newproxy.com".to_string(), ProxyHostConfig {
                            proxy_addr: "127.0.0.1:9001".to_string(),
                            proxy_tls: false,
                            proxy_uds: None,
                            proxy_headers: Some(vec![(
                                "X-New-Proxy-Header".to_string(),
                                "new-proxy-value".to_string(),
                            )]),
                            routes: Some(HashMap::from([
                                ("/dashboard".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(false),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                                ("/settings".to_string(), ProxyPathBaseHostConfig {
                                    proxy_addr: None,
                                    proxy_tls: Some(false),
                                    proxy_uds: None,
                                    proxy_headers: None,
                                }),
                            ])),
                        }),
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
                        ("example.com".to_string(), LBHostConfig {
                            load_balancer_tls: false,
                            load_balancer_headers: Some(vec![(
                                "X-LB-Example".to_string(),
                                "value".to_string(),
                            )]),
                        }),
                        ("another.com".to_string(), LBHostConfig {
                            load_balancer_tls: true,
                            load_balancer_headers: Some(vec![(
                                "X-LB-Another".to_string(),
                                "another-value".to_string(),
                            )]),
                        }),
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
                        ("proxyexample.com".to_string(), LBHostConfig {
                            load_balancer_tls: true,
                            load_balancer_headers: Some(vec![(
                                "X-Proxy-LB-Header".to_string(),
                                "proxy-lb-value".to_string(),
                            )]),
                        }),
                        ("newproxy.com".to_string(), LBHostConfig {
                            load_balancer_tls: false,
                            load_balancer_headers: Some(vec![(
                                "X-New-Proxy-LB-Header".to_string(),
                                "new-proxy-lb-value".to_string(),
                            )]),
                        }),
                    ]),
                },
            ]),
        }
    }

    pub fn new_proxy_only() -> Self {
        let mut proxy_servers1 = HashMap::new();
        proxy_servers1.insert("server1".to_string(), ProxyHostConfig {
            proxy_addr: "/tmp/proxy1.sock".to_string(),
            proxy_tls: true,
            proxy_uds: Some(true),
            proxy_headers: Some(vec![("Header1".to_string(), "Value1".to_string())]),
            routes: Some(HashMap::from([
                ("/route1".to_string(), ProxyPathBaseHostConfig {
                    proxy_addr: None,
                    proxy_tls: Some(true),
                    proxy_uds: None,
                    proxy_headers: None,
                }),
                ("/route2".to_string(), ProxyPathBaseHostConfig {
                    proxy_addr: None,
                    proxy_tls: Some(true),
                    proxy_uds: None,
                    proxy_headers: None,
                }),
            ])),
        });

        let mut proxy_servers2 = HashMap::new();
        proxy_servers2.insert("server2".to_string(), ProxyHostConfig {
            proxy_addr: "192.168.1.2".to_string(),
            proxy_tls: false,
            proxy_uds: None,
            proxy_headers: Some(vec![("Header2".to_string(), "Value2".to_string())]),
            routes: Some(HashMap::from([
                ("/home".to_string(), ProxyPathBaseHostConfig {
                    proxy_addr: None,
                    proxy_tls: Some(false),
                    proxy_uds: None,
                    proxy_headers: None,
                }),
                ("/status".to_string(), ProxyPathBaseHostConfig {
                    proxy_addr: None,
                    proxy_tls: Some(false),
                    proxy_uds: None,
                    proxy_headers: None,
                }),
            ])),
        });

        Self {
            prometheus_addr: None,
            proxy: Some(vec![
                ProxyConfig {
                    listener: "127.0.0.1:8080".to_string(),
                    tls_certificate: Some("path/to/cert1".to_string()),
                    tls_certificate_key: Some("path/to/key1".to_string()),
                    servers: proxy_servers1,
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

    pub fn new_load_balancer_only() -> Self {
        let mut lb_servers1 = HashMap::new();
        lb_servers1.insert("lb1".to_string(), LBHostConfig {
            load_balancer_tls: true,
            load_balancer_headers: Some(vec![("Header1".to_string(), "Value1".to_string())]),
        });

        let mut lb_servers2 = HashMap::new();
        lb_servers2.insert("lb2".to_string(), LBHostConfig {
            load_balancer_tls: false,
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
                    servers: lb_servers1,
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
