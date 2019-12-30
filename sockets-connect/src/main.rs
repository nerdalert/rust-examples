/// Example for setting a timeout value on socket connections
/// and measure the time it takes to make the connection.
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

fn main() {
    // This variable will determine the timeout value for our socket
    let socket_timeout = Duration::new(5, 0);
    let remote: SocketAddr = "1.1.1.1:80".parse().unwrap();

    // Take a timestamp prior to the socket opening
    let timestamp_start = Instant::now();

    // Connect to the socket with a timeout value of 5 seconds
    if TcpStream::connect_timeout(&remote, socket_timeout).is_ok() {
        println!("Successfully connected to {:?}", remote);
    } else {
        println!("Failed connection to {:?}", remote);
    }

    // Take a timestamp after the socket closed
    let timestamp_end = Instant::now();
    // Print the time it took to open a socket to the target
    println!("Latency to host: {:?} was {:?}\n", &remote, timestamp_end - timestamp_start);

    // Now lets test a host that will timeout
    let remote: SocketAddr = "8.8.8.8:80".parse().unwrap();
    if TcpStream::connect_timeout(&remote, socket_timeout).is_ok() {
        println!("Successfully connected to {:?}", remote);
    } else {
        println!("Connection to {:?} timed out after {:?}", remote, socket_timeout);
    }
}

