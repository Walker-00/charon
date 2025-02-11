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

## Performance Benchmark

Charon has been benchmarked to handle up to **64,439** concurrent requests, outperforming other proxy servers like **Nginx**, which handles around **17,744** concurrent requests. This means **Charon** is approximately **263% faster** than **Nginx**, making it a highly efficient solution for high-performance applications.

![image](https://github.com/user-attachments/assets/a3993c51-8e85-41d8-a600-af74da75b8cb)


## Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed. Then, clone the repository and run the following command:

```bash
cargo build --release
```

## Configuration

The configuration file uses TOML format and supports specifying proxy and load balancer settings.

### Example Configuration

Both [TOML](https://toml.io) and my own config language [ACHERON](https://github.com/Walker-00/acheron) is supported.

Check the [example](https://github.com/Walker-00/charon/tree/rust/example) folder for more config.

ACHERON example:

```toml
prometheus_addr = "0.0.0.0:9090"

[[proxy]]
listener = "0.0.0.0:8080"

[[host="example.com"]]
proxy_addr = "/tmp/example.sock"
proxy_tls = false
proxy_headers = [
    ["X-Example-Header", "value"]
]
proxy_uds = true

[[[route="/api/v1"]]]
proxy_addr = "/tmp/example.sock"
proxy_tls = false
proxy_uds = true

[[[route="/status"]]]
proxy_tls = false

[[host="another.com"]]
proxy_addr = "127.0.0.1:8001"
proxy_tls = true
proxy_headers = [
    ["X-Another-Header", "another-value"]
]

[[[route="/login"]]]
proxy_tls = true

[[[route="/home"]]]
proxy_tls = true

[[proxy]]
listener = "0.0.0.0:9090"
tls_certificate = "cert.pem"
tls_certificate_key = "key.pem"

[[host="newproxy.com"]]
proxy_addr = "127.0.0.1:9001"
proxy_tls = false
proxy_headers = [
    ["X-New-Proxy-Header", "new-proxy-value"]
]

[[[route="/dashboard"]]]
proxy_tls = false

[[[route="/settings"]]]
proxy_tls = false

[[host="proxyexample.com"]]
proxy_addr = "/tmp/proxy.sock"
proxy_tls = true
proxy_headers = [
    ["X-Proxy-Header", "proxy-value"]
]
proxy_uds = true

[[[route="/info"]]]
proxy_tls = true

[[[route="/data"]]]
proxy_tls = true

[[load_balancer]]
listener = "0.0.0.0:7070"
upstreams = [
    "127.0.0.1:7000",
    "127.0.0.1:7001"
]
health_check = true
health_check_frequency = 30
parallel_health_check = true

[[host="example.com"]]
load_balancer_tls = false
load_balancer_headers = [
    ["X-LB-Example", "value"]
]

[[host="another.com"]]
load_balancer_tls = true
load_balancer_headers = [
    ["X-LB-Another", "another-value"]
]

[[load_balancer]]
listener = "0.0.0.0:8080"
upstreams = [
    "127.0.0.1:8081",
    "127.0.0.1:8082"
]
health_check = false
parallel_health_check = false
tls_certificate = "loadbalancer_cert.pem"
tls_certificate_key = "loadbalancer_key.pem"

[[host="proxyexample.com"]]
load_balancer_tls = true
load_balancer_headers = [
    ["X-Proxy-LB-Header", "proxy-lb-value"]
]

[[host="newproxy.com"]]
load_balancer_tls = false
load_balancer_headers = [
    ["X-New-Proxy-LB-Header", "new-proxy-lb-value"]
]
```


TOML example:

```toml
prometheus_addr = "0.0.0.0:9090"

[[proxy]]
listener = "0.0.0.0:8080"

[proxy.servers."example.com"]
proxy_addr = "/tmp/example.sock"
proxy_tls = false
proxy_headers = [[
    "X-Example-Header",
    "value",
]]
proxy_uds = true

[proxy.servers."example.com".routes."/api/v1"]
proxy_addr = "/tmp/example.sock"
proxy_tls = false
proxy_uds = true

[proxy.servers."example.com".routes."/status"]
proxy_tls = false

[proxy.servers."another.com"]
proxy_addr = "127.0.0.1:8001"
proxy_tls = true
proxy_headers = [[
    "X-Another-Header",
    "another-value",
]]

[proxy.servers."another.com".routes."/login"]
proxy_tls = true

[proxy.servers."another.com".routes."/home"]
proxy_tls = true

[[proxy]]
listener = "0.0.0.0:9090"
tls_certificate = "cert.pem"
tls_certificate_key = "key.pem"

[proxy.servers."newproxy.com"]
proxy_addr = "127.0.0.1:9001"
proxy_tls = false
proxy_headers = [[
    "X-New-Proxy-Header",
    "new-proxy-value",
]]

[proxy.servers."newproxy.com".routes."/dashboard"]
proxy_tls = false

[proxy.servers."newproxy.com".routes."/settings"]
proxy_tls = false

[proxy.servers."proxyexample.com"]
proxy_addr = "/tmp/proxy.sock"
proxy_tls = true
proxy_headers = [[
    "X-Proxy-Header",
    "proxy-value",
]]
proxy_uds = true

[proxy.servers."proxyexample.com".routes."/info"]
proxy_tls = true

[proxy.servers."proxyexample.com".routes."/data"]
proxy_tls = true

[[load_balancer]]
listener = "0.0.0.0:7070"
upstreams = [
    "127.0.0.1:7000",
    "127.0.0.1:7001",
]
health_check = true
health_check_frequency = 30
parallel_health_check = true

[load_balancer.servers."example.com"]
load_balancer_tls = false
load_balancer_headers = [[
    "X-LB-Example",
    "value",
]]

[load_balancer.servers."another.com"]
load_balancer_tls = true
load_balancer_headers = [[
    "X-LB-Another",
    "another-value",
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

[load_balancer.servers."proxyexample.com"]
load_balancer_tls = true
load_balancer_headers = [[
    "X-Proxy-LB-Header",
    "proxy-lb-value",
]]

[load_balancer.servers."newproxy.com"]
load_balancer_tls = false
load_balancer_headers = [[
    "X-New-Proxy-LB-Header",
    "new-proxy-lb-value",
]]

```

### Command Line Arguments

- `--config <path>`: Path to the configuration file.
- `--example`: Prints the example configuration file (Only TOML for now).
- `--example_proxy`: Prints the example proxy configuration (Only TOML for now).
- `--example_load_balancer`: Prints the example load balancer configuration (Only TOML for now).

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
