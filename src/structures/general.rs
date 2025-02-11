use serde::{Deserialize, Serialize};

use super::{load_balancer_structure::LoadBalancerConfig, proxy_structure::ProxyConfig};

#[derive(clap::Parser, Debug)]
#[command(
    version,
    about = "Charon: The Proxy Server",
    long_about = "Charon is a proxy server, built on Pingora, that ferries packets across the digital river—transferring data from the chaotic internet to servers, much like the mythical Charon guided souls to the underworld."
)]
pub struct Args {
    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<String>,

    /// Get Example Full Config
    #[arg(short = 'e', long)]
    pub example: bool,

    /// Get Example Proxy Config
    #[arg(short = 'p', long)]
    pub example_proxy: bool,

    /// Get Example Load Balancer Config
    #[arg(short = 'l', long)]
    pub example_load_balancer: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub prometheus_addr: Option<String>,
    pub proxy: Option<Vec<ProxyConfig>>,
    pub load_balancer: Option<Vec<LoadBalancerConfig>>,
}
