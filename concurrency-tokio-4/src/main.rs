use futures::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use std::net::SocketAddr;

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 1024];

    loop {
        match stream.read(&mut buf).await? {
            0 => break, // Socket closed
            n => {
                // Send the data back
                stream.write_all(&buf[0..n]).await?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;

    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8000".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    // Bind the TCP listener
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    #[allow(irrefutable_let_patterns)]
    while let (stream, _) = listener.accept().await? {
        tokio::spawn(handle_client(stream).map(|_| ()));
    }
    Ok(())
}
