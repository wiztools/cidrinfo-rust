use std::env;
use std::net::Ipv4Addr;
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <cidr>", args[0]);
        process::exit(1);
    }

    let cidr = &args[1];

    match parse_cidr(cidr) {
        Ok((ip, prefix_len)) => {
            display_cidr_info(ip, prefix_len);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn parse_cidr(cidr: &str) -> Result<(Ipv4Addr, u8), String> {
    let parts: Vec<&str> = cidr.split('/').collect();

    if parts.len() != 2 {
        return Err("Invalid CIDR format. Expected format: x.x.x.x/y".to_string());
    }

    let ip = match Ipv4Addr::from_str(parts[0]) {
        Ok(ip) => ip,
        Err(_) => return Err("Invalid IP address".to_string()),
    };

    let prefix_len = match parts[1].parse::<u8>() {
        Ok(len) if len <= 32 => len,
        _ => return Err("Invalid prefix length. Must be between 0 and 32".to_string()),
    };

    Ok((ip, prefix_len))
}

fn display_cidr_info(ip: Ipv4Addr, prefix_len: u8) {
    // Calculate network address (apply the subnet mask)
    let ip_u32 = u32::from(ip);
    let mask = if prefix_len == 0 {
        0
    } else {
        !0u32 << (32 - prefix_len)
    };
    let network_addr = Ipv4Addr::from(ip_u32 & mask);

    // Calculate netmask in dotted decimal format
    let netmask = Ipv4Addr::from(mask);

    // Calculate the first and last IP in the range
    let first_ip = network_addr;
    let last_ip_u32 = ip_u32 | !mask;
    let last_ip = Ipv4Addr::from(last_ip_u32);

    // Calculate total number of IPs in this subnet
    let count = 2u32.pow((32 - prefix_len) as u32);

    // Determine the type of IP address
    let ip_type = determine_ip_type(&network_addr);

    // Display the information
    println!("Network:     {}/{}", network_addr, prefix_len);
    println!("Netmask:     {}", netmask);
    println!("CIDR Range:  {}  <-to->  {}", first_ip, last_ip);
    println!("Count:       {}", count);
    println!("Type:        {}", ip_type);
}

fn determine_ip_type(ip: &Ipv4Addr) -> &'static str {
    if ip.is_private() {
        "private"
    } else if ip.is_loopback() {
        "loopback"
    } else if ip.is_link_local() {
        "link-local"
    } else if ip.is_broadcast() {
        "broadcast"
    } else if ip.is_multicast() {
        "multicast"
    } else if ip.is_unspecified() {
        "unspecified"
    } else {
        "public"
    }
}
