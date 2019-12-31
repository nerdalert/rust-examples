/// This lib appears unmaintained. Had to drop it back a release
/// to get it to build. Enough to get a sample netlink call across.
/// This example will only compile on Linux machines.
extern crate pnetlink;

use pnetlink::packet::netlink::NetlinkConnection;
use pnetlink::packet::route::addr::{Addr, Addresses};
use pnetlink::packet::route::link::{Link, Links};

fn print_link(link: &Link) {
    println!(
        "{}: {}: <{:?}> mtu {:?} qdisc {} state {:?}",
        link.get_index(),
        link.get_name().unwrap(),
        link.get_flags(),
        link.get_mtu().unwrap(),
        link.get_qdisc().unwrap(),
        link.get_state()
    );
    println!(
        "   Link/{:?} {:?} brd {:?}",
        link.get_type(),
        link.get_hw_addr().unwrap(),
        link.get_broadcast().unwrap()
    );
}

fn print_addr(addr: &Addr) {
    let family = match addr.get_family() {
        2 => "inet",
        10 => "inet6",
        _ => "unknown",
    };
    fn debug_or_empty<T: ::std::fmt::Debug>(prefix: &str, val: Option<T>) -> String {
        match val {
            Some(val) => format!("{}{:?} ", prefix, val),
            None => " ".to_owned(),
        }
    }
    fn display_or_empty<T: ::std::fmt::Display>(prefix: &str, val: Option<T>) -> String {
        match val {
            Some(val) => format!("{}{} ", prefix, val),
            None => " ".to_owned(),
        }
    }
    let broadcast = debug_or_empty("brd ", addr.get_broadcast_ip());
    let label = display_or_empty("", addr.get_label());
    println!(
        "   {} {:?}/{}{}scope {:?} {}",
        family,
        addr.get_ip().unwrap(),
        addr.get_prefix_len(),
        broadcast,
        addr.get_scope(),
        label
    );
}

fn main() {
    let mut conn = NetlinkConnection::new();
    let links = conn.iter_links().unwrap().collect::<Vec<_>>();
    for link in links {
        print_link(&link);
        for addr in conn.get_link_addrs(None, &link).unwrap() {
            //println!("{:?}", addr.get_ip());
            print_addr(&addr);
        }
    }
}

/*
Example output (linux only):
---------------------------
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: ens33: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether 00:0c:29:58:7f:4c brd ff:ff:ff:ff:ff:ff
6: dummy1: <BROADCAST,NOARP> mtu 1500 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/ether 8a:86:af:cd:36:0d brd ff:ff:ff:ff:ff:ff
*/
