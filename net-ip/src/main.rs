use std::net::IpAddr;

fn main() {
    // convert an IPv4 string representation to IPv4
    let ip4_string = "127.0.0.1";
    let ip_v4: std::net::IpAddr = ip4_string.parse().unwrap();
    println!(
        "Parsed IPv4 string {} to net:IpAddr -> {:?}",
        ip4_string, ip_v4
    );

    // convert an IPv6 string representation to IPv6
    let ip6_string = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
    let ip_v4: std::net::IpAddr = ip6_string.parse().unwrap();
    println!(
        "Parsed IPv6 string {} to net:IpAddr -> {:?}",
        ip6_string, ip_v4
    );

    // Error handling a bad IPv4 lookup
    let good_v4_parse = parse_ip_result(ip4_string);
    if good_v4_parse.is_ok() {
        println!(
            "Parsed IPv4 string {} to net:IpAddr -> {:?}",
            ip4_string, good_v4_parse
        );
    } else {
        println!(
            "Error looking up name {} Error -> {:?}",
            ip4_string, good_v4_parse
        );
    }

    // Error handling a bad IPv4 lookup
    let bad_ip4_string = "127.34.232.0.0.1";
    let bad_v4_parse = parse_ip_result(bad_ip4_string);
    if bad_v4_parse.is_ok() {
        println!(
            "Parsed IPv4 string {} to net:IpAddr -> {:?}",
            bad_ip4_string, bad_v4_parse
        );
    } else {
        println!(
            "Error looking up name {} Error -> {:?}",
            bad_ip4_string, bad_v4_parse
        );
    }
}

fn parse_ip_result(txt: &str) -> Result<IpAddr, String> {
    return match txt.parse::<IpAddr>() {
        Ok(val) => Ok(val),
        Err(err) => Err(format!(
            "Could not parse IP from {} because of {}",
            txt, err
        )),
    };
}
