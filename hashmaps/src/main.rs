/// Where vectors store values by an integer index, HashMaps
/// store values by key. HashMap keys can be booleans,
/// integers, strings, or any other type that implements the
/// Eq and Hash traits. Like vectors, HashMaps are growable,
/// but HashMaps can also shrink themselves when they have
/// excess space.
use std::collections::HashMap;

fn main() {
    let mut hosts = HashMap::new();
    hosts.insert("server1", "172.17.0.10");
    hosts.insert("server2", "192.168.100.10");
    hosts.insert("server3", "10.100.10.20");
    hosts.insert("server4", "10.200.10.30");
    // list the map count
    println!("Server count -> {}", hosts.len());
    // get the val from server1
    let srv1_ip = hosts.get("server1");
    println!("The IP for server1 is -> {:?}", srv1_ip.unwrap());
    // overwrite the val for the server1 key
    hosts.insert("server1", "192.168.200.40");
    let srv1_ip = hosts.get("server1");
    println!("The new IP for server1 is -> {:?}", srv1_ip.unwrap());
    // iterate over the map
    for (k, v) in &hosts {
        println!("{} : {}", k, v);
    }
}

/*
The output is:
-------------
Server count -> 4
The IP for server1 is -> "172.17.0.10"
The new IP for server1 is -> "192.168.200.40"
server1 : 192.168.200.40
server3 : 10.100.10.20
server4 : 10.200.10.30
server2 : 192.168.100.10
*/
