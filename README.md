# cidrinfo

A simple command-line utility that provides detailed information about IP addresses in CIDR notation.

## Overview

`cidrinfo` takes an IP address in CIDR notation (e.g., `192.168.1.0/24`) and displays useful network information including:

- Network address
- Netmask in decimal format
- IP range (first and last addresses)
- Total count of addresses in the range
- IP type classification (private, public, loopback, etc.)

## Installation

### From Source

1. Ensure you have Rust and Cargo installed (https://rustup.rs/)
2. Clone this repository:
   ```
   git clone https://github.com/wiztools/cidrinfo-rust.git
   cd cidrinfo-rust
   ```
3. Build the project:
   ```
   cargo build --release
   ```
4. The binary will be available at `./target/release/cidrinfo`
5. (Optional) Install to your system:
   ```
   cargo install --path .
   ```

## Usage

```
cidrinfo <ip-cidr>
```

Where `<ip-cidr>` is an IPv4 address in CIDR notation.

### Examples

```bash
# Display information for a private network
$ cidrinfo 10.16.0.0/28

Network:     10.16.0.0/28
Netmask:     255.255.255.240
CIDR Range:  10.16.0.0  <-to->  10.16.0.15
Count:       16
Type:        is-private

# Display information for a different subnet
$ cidrinfo 192.168.1.0/24

Network:     192.168.1.0/24
Netmask:     255.255.255.0
CIDR Range:  192.168.1.0  <-to->  192.168.1.255
Count:       256
Type:        is-private

# Display information for a public IP
$ cidrinfo 203.0.113.0/29

Network:     203.0.113.0/29
Netmask:     255.255.255.248
CIDR Range:  203.0.113.0  <-to->  203.0.113.7
Count:       8
Type:        is-public
```

## IP Type Classifications

The tool identifies various network types:

- `is-private`: RFC 1918 private networks (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
- `is-loopback`: Loopback addresses (127.0.0.0/8)
- `is-link-local`: Link-local addresses (169.254.0.0/16)
- `is-broadcast`: Broadcast address (255.255.255.255)
- `is-multicast`: Multicast addresses (224.0.0.0/4)
- `is-unspecified`: Unspecified address (0.0.0.0)
- `is-public`: Public IP addresses (any address not falling into the above categories)

## Error Handling

The tool provides helpful error messages for invalid inputs:

```bash
# Invalid CIDR format
$ cidrinfo 192.168.1.0

Error: Invalid CIDR format. Expected format: x.x.x.x/y

# Invalid IP address
$ cidrinfo 999.168.1.0/24

Error: Invalid IP address

# Invalid prefix length
$ cidrinfo 192.168.1.0/33

Error: Invalid prefix length. Must be between 0 and 32
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
