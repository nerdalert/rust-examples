/// To run this example, first open a socket listening on
/// port 8000 (or whatever you want to change it to by
/// specifying the port in 127.0.0.1:8000).
/// Next, using nc/netcat open a listener. On a Mac for
/// example, you can run `nc -lvp 8000`.
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::prelude::*;

const MESSAGES: &[&str] = &["hello", "world", "goodbye"];

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
        // check if the bytes match
        if &buf[..msg.len()] == msg.as_bytes() {
            println!("The messages matched!");
            println!("Left  {:?}\nRight {:?}", &buf[..msg.len()], msg.as_bytes());
        } else {
            println!("The messages did not match!");
            println!("Left  {:?}\nRight {:?}", &buf[..msg.len()], msg.as_bytes());
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8000".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    // Connect to the echo server
    match run_client(addr).await {
        Ok(_) => println!("done."),
        Err(e) => eprintln!("echo client failed; error = {:?}", e),
    }

    Ok(())
}

/*
Example output:
--------------
In a seperate terminal start a netcat listener:
nc -lvp 8000

Then run the app:
$> cargo run
-- output --
Connected
 > write = "hello"
 -----------

 In the netcat window you should see the sent message.
 You can then type the exact message back in the netcat
 window to be decoded by the app.

 If you type the same word back from the netcat window:
  > write = "hello"
The messages matched!
Left  [104, 101, 108, 108, 111]
Right [104, 101, 108, 108, 111]

The messages did not match!
Left  [102, 111, 111, 10, 111]
Right [119, 111, 114, 108, 100]
*/
