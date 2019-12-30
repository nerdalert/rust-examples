use dns_lookup::lookup_addr;
use dns_lookup::lookup_host;
use std::net::IpAddr;

fn main() {
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
    let another_lookup = resolve_host(hostname_lookup);
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
    let another_lookup = resolve_host(hostname_lookup);
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

    // Shorter handling
    let hostname_lookup_goog = "123.5.2.2.2";
    let ip_results = resolve_host(hostname_lookup_goog);
    println!("Results: {:?}", ip_results);
}

// resolve a hostname to IpAddr
fn resolve_host(host: &str) -> Result<IpAddr, String> {
    match host.parse::<IpAddr>() {
        Ok(val) => Ok(val),
        _ => {
            let ip_list = lookup_host(host).map_err(|_| "dns_lookup::lookup_host failed")?;
            Ok(*ip_list.first().unwrap())
        }
    }
}

fn parse_ip_result(txt: &str) -> Result<IpAddr, String> {
    match txt.parse::<IpAddr>() {
        Ok(val) => Ok(val),
        Err(err) => Err(format!(
            "Could not parse IP from {} because of {}",
            txt, err
        )),
    }
}

/*
Example Results:
---------------
Lookup results for 127.0.0.1 -> localhost
Lookup results for google.com -> [V4(172.217.8.206)]
Lookup results for google.com -> V4(172.217.8.206)
Lookup results for google.com -> Ok(V4(172.217.8.206))
Error looking up name sadf1234.com Error -> Err("dns_lookup::lookup_host failed")
Results: Err("dns_lookup::lookup_host failed")
*/
