use futures::StreamExt;
use std::{env, error::Error, net::SocketAddr};
use tokio::{
    self,
    codec::{self, Framed},
    net::{TcpListener, TcpStream},
};

pub struct LogListener {
    listen_bind: String,
}

type AsyncRes = Result<(), Box<dyn Error>>;

impl LogListener {
    pub fn new(listen_bind: String) -> Self {
        Self { listen_bind }
    }

    pub async fn setup(&mut self) -> AsyncRes {
        let addr: SocketAddr = self.listen_bind.parse().unwrap();
        let mut listener = TcpListener::bind(&addr).await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(_) = Self::process(stream, addr).await {
                    println!("Error: stream proccess error");
                }
            });
        }
    }

    pub async fn process(stream: TcpStream, addr: SocketAddr) -> AsyncRes {
        let mut stream = Framed::new(stream, codec::LinesCodec::new());
        println!("New client connected from {}", &addr);
        loop {
            match stream.next().await {
                Some(Ok(msg)) => {
                    println!("{}: {}", &addr, msg);
                }
                Some(Err(_)) => {
                    println!("Connection error from {}", &addr);
                    return Ok(());
                }
                None => {
                    println!("Lost connection with {}", &addr);
                    return Ok(());
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> AsyncRes {
    let mut listen_bind = "0.0.0.0:2000".to_string();
    for arg in env::args().skip(1) {
        if arg.starts_with("-l") {
            listen_bind = arg.split_at(3).1.to_string();
            println!("listening on {}", listen_bind);
        }
    }

    let mut listener = LogListener::new(listen_bind);
    listener.setup().await?;
    Ok(())
}
