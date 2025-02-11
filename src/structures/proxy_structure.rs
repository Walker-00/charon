use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::proxy::service::ProxyHostConfig;

#[derive(Serialize, Deserialize)]
pub struct ProxyConfig {
    pub listener: String,
    pub tls_certificate: Option<String>,
    pub tls_certificate_key: Option<String>,
    pub servers: HashMap<String, ProxyHostConfig>,
}
