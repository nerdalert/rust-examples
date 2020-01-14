//! TCP echo server.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc localhost 8080
//! ```

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn process(stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);
    let (reader, writer) = &mut (&stream, &stream);
    // Welcome msg
    writer.write(b"Hello Frens\n").await?;
    // Copy the incoming reader to the outgoing writer
    io::copy(reader, writer).await?;
    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}

/*
Example output:
--------------
Listening on 127.0.0.1:8080
Accepted from: 127.0.0.1:52695
Accepted from: 127.0.0.1:52720
*/
