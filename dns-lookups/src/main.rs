use dns_lookup::lookup_addr;
use dns_lookup::lookup_host;
use std::net::IpAddr;

fn main() {
    println!("Hello, world!");

    // Resolv an IPv4 string representation to IPv4
    let ip_lookup = "127.0.0.1";
    let ip: std::net::IpAddr = ip_lookup.parse().unwrap();
    let host = lookup_addr(&ip).unwrap();
    println!("Lookup results for {} -> {}", ip_lookup, host);

    // Resolv a DNS name to IPv4
    let hostname_lookup = "google.com";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname_lookup).unwrap();
    println!("Lookup results for {} -> {:?}", hostname_lookup, ips);
    for ip in ips.iter() {
        println!("Lookup results for {} -> {:?}", hostname_lookup, ip);
    }

    // A working lookup with error handling
    let another_lookup = resolve(hostname_lookup);
    if another_lookup.is_ok() {
        println!(
            "Lookup results for {} -> {:?}",
            hostname_lookup, another_lookup
        );
    } else {
        println!(
            "Error looking up name {} Error -> {:?}",
            hostname_lookup, another_lookup
        );
    }

    // A bad lookup with error handling
    let hostname_lookup = "sadf1234.com";
    let another_lookup = resolve(hostname_lookup);
    if another_lookup.is_ok() {
        println!(
            "Lookup results for {} -> {:?}",
            hostname_lookup, another_lookup
        );
    } else {
        println!(
            "Error looking up name {} Error -> {:?}",
            hostname_lookup, another_lookup
        );
    }
}

// resolve a hostname to IpAddr
fn resolve(host: &str) -> Result<IpAddr, String> {
    let ip_list = dns_lookup::lookup_host(host).map_err(|_| "dns_lookup::lookup_host")?;
    Ok(ip_list.first().unwrap().clone())
}
