/// Using the built in lookups for DNS resolution in
use std::net::TcpStream;

fn main() {
    println!("\nExample #1 with an unresolvable hostname:");
    match TcpStream::connect("wbroken-address.com:80") {
        Ok(_) => {
            println!("connected");
            // break;
        }
        Err(e) => {
            println!("failed: {:?}", e);
        }
    }
    println!("\nExample #2 with a resolvable hostname:");
    match TcpStream::connect("google.com:80") {
        Ok(_) => {
            println!("connected to google.com:80");
            // break;
        }
        Err(e) => {
            println!("failed: {:?}", e);
        }
    }
}

/*
Example output:
--------------
Example #1 with an unresolvable hostname:
failed: Custom { kind: Other, error: "failed to lookup address information: nodename nor servname provided, or not known" }

Example #2 with a resolvable hostname:
connected to google.com:80
*/
