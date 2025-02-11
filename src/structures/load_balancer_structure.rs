use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::load_balancer::service::LBHostConfig;

#[derive(Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    pub listener: String,
    pub upstreams: Vec<String>,
    pub health_check: Option<bool>,
    pub health_check_frequency: Option<u64>,
    pub parallel_health_check: Option<bool>,
    pub tls_certificate: Option<String>,
    pub tls_certificate_key: Option<String>,
    pub servers: HashMap<String, LBHostConfig>,
}
