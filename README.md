<div align="center">

<img src="https://github.com/user-attachments/assets/c4459ede-4b98-4605-830f-3c00704f907f" width="350" height="350" alt="Charon" />

</div>

# Charon: The Proxy Server

Charon is a proxy server, built on Pingora, designed to transfer data from the chaotic internet to servers, similar to how the mythical Charon ferried souls across the river Styx. It includes support for proxy and load balancing services with configurable TLS support, routing, and health checks.

```text
In the shadowed silence of the digital expanse, there exists a ferryman—Charon.
Forged in Rust and tempered by the flow of endless requests, Charon stands at the riverbanks of the interwebs, offering passage across its turbulent streams.
Every packet, every connection, every whispered request is but a coin placed in his outstretched hand.
Do as you wish, traverse the endless waters, command the tides of data—but remember,
in the end, you are nothing more than a passenger, crossing the river of interwebs under the watchful gaze of Charon.
```

## Features

- **Proxy Service:** Provides HTTP proxy functionality, routing requests to different hosts based on the `Host` header in the request.
- **Load Balancer Service:** Implements a load balancer with Round Robin selection and support for multiple upstream servers. 
- **TLS Support:** Configurable TLS for secure communication. You can specify a certificate and key for secure connections.
- **Health Checks:** Configurable health checks for load balancer upstreams to ensure availability.
- **Configurable Headers:** Ability to inject headers into requests based on the host configuration.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, clone the repository and run the following command:

```bash
cargo build --release
```

## Configuration

The configuration file uses TOML format and supports specifying proxy and load balancer settings.

### Example Configuration

check [example](https://github.com/Walker-00/charon/tree/rust/example) folder for more config.

```toml
[[proxy]]
listener = "0.0.0.0:8080"

[proxy.servers."example.com"]
proxy_addr = "127.0.0.1:8000"
proxy_tls = false
proxy_hostname = "example.com"
proxy_headers = [[
    "X-Example-Header",
    "value",
]]

[proxy.servers."another.com"]
proxy_addr = "127.0.0.1:8001"
proxy_tls = true
proxy_hostname = "another.com"
proxy_headers = [[
    "X-Another-Header",
    "another-value",
]]

[[proxy]]
listener = "0.0.0.0:9090"
tls_certificate = "cert.pem"
tls_certificate_key = "key.pem"

[proxy.servers."newproxy.com"]
proxy_addr = "127.0.0.1:9001"
proxy_tls = false
proxy_hostname = "newproxy.com"
proxy_headers = [[
    "X-New-Proxy-Header",
    "new-proxy-value",
]]

[proxy.servers."proxyexample.com"]
proxy_addr = "127.0.0.1:9000"
proxy_tls = true
proxy_hostname = "proxyexample.com"
proxy_headers = [[
    "X-Proxy-Header",
    "proxy-value",
]]

[[load_balancer]]
listener = "0.0.0.0:7070"
upstreams = [
    "127.0.0.1:7000",
    "127.0.0.1:7001",
]
health_check = true
health_check_frequency = 30
parallel_health_check = true

[load_balancer.servers."another.com"]
load_balancer_tls = true
load_balancer_hostname = "another.com"
load_balancer_headers = [[
    "X-LB-Another",
    "another-value",
]]

[load_balancer.servers."example.com"]
load_balancer_tls = false
load_balancer_hostname = "example.com"
load_balancer_headers = [[
    "X-LB-Example",
    "value",
]]

[[load_balancer]]
listener = "0.0.0.0:8080"
upstreams = [
    "127.0.0.1:8081",
    "127.0.0.1:8082",
]
health_check = false
parallel_health_check = false
tls_certificate = "loadbalancer_cert.pem"
tls_certificate_key = "loadbalancer_key.pem"

[load_balancer.servers."newproxy.com"]
load_balancer_tls = false
load_balancer_hostname = "newproxy.com"
load_balancer_headers = [[
    "X-New-Proxy-LB-Header",
    "new-proxy-lb-value",
]]

[load_balancer.servers."proxyexample.com"]
load_balancer_tls = true
load_balancer_hostname = "proxyexample.com"
load_balancer_headers = [[
    "X-Proxy-LB-Header",
    "proxy-lb-value",
]]
```

### Command Line Arguments

- `--config <path>`: Path to the configuration file.
- `--example`: Prints the example configuration file.
- `--example_proxy`: Prints the example proxy configuration.
- `--example_load_balancer`: Prints the example load balancer configuration.

## Usage

Run the server with the following command:

```bash
charon --config <file_path>
```

This will start the server with the specified configuration file.

## License

Charon is licensed under the **WTFPL (Do What The F*ck You Want To Public License)**.

<a href="http://www.wtfpl.net/"><img
       src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl.svg"
       width="220" height="160" alt="WTFPL" /></a>

## Contributing

Feel free to open issues or submit pull requests for improvements.
