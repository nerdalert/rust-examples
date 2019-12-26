use tokio::net::TcpStream;
use tokio::prelude::*;
use std::net::SocketAddr;

const MESSAGES: &[&str] = &["hello", "world", "one two three"];

async fn run_client(addr: SocketAddr) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(&addr).await?;
    println!("Connected");

    // Buffer to read into
    let mut buf = [0; 128];

    for msg in MESSAGES {
        println!(" > write = {:?}", msg);

        // Write the message to the server
        stream.write_all(msg.as_bytes()).await?;

        // Read the message back from the server
        stream.read(&mut buf).await?;
        assert_eq!(&buf[..msg.len()], msg.as_bytes());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8000".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    // Connect to the echo server
    match run_client(addr).await {
        Ok(_) => println!("done."),
        Err(e) => eprintln!("echo client failed; error = {:?}", e),
    }

    Ok(())
}
