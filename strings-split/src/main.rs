fn main() {
    let split_me = String::from("127.0.0.1:80");
    let p = String::from(":");
    // Split on the ':' pattern
    if split_me.contains(&p) {
        let mut split = split_me.split(&p);
        let ip = split.next().unwrap();
        let port = split.next().unwrap();
        println!("IP -> {}", ip);
        println!("Port -> {}", port);
    } else {
        println!("Pattern to split on '{}' not found", p);
    }
}

/*
Example output:
--------------
IP -> 127.0.0.1
Port -> 80
*/
