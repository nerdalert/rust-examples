/// This is just a toy BGP listener Im playing around with
/// to learn tokio 0.2.0+
extern crate futures;
extern crate tokio;
extern crate tokio_timer;

use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::{
    codec::{Framed, LinesCodec},
    net::{TcpListener, TcpStream},
    prelude::*,
    sync::{mpsc, Mutex},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hacking BGP Listener");

    let mut table = Table::new();
    let table = Arc::new(Mutex::new(table));
    let (init_tx, mut init_rx) = mpsc::channel::<()>(1);
    let global = Arc::new(Mutex::new(Global::new(init_tx)));
    let addr = "[::]:179".to_string();
    let mut listener = TcpListener::bind(&addr).await?;
    let f = |sock: std::net::SocketAddr| -> IpAddr {
        let mut addr = sock.ip();
        if let IpAddr::V6(a) = addr {
            if let Some(a) = a.to_ipv4() {
                addr = IpAddr::V4(a);
            }
        }
        addr
    };
    loop {
        // let (stream, addr) = listener.accept().await?;
        let (stream, sock) = listener.accept().await?;
        let addr = f(sock);
        println!(
            "Received new connection address --> {:?} Is_V6 --> {}",
            addr,
            addr.is_ipv6()
        );
        if let Ok(l) = stream.local_addr() {
            let global = Arc::clone(&global);
            let table = Arc::clone(&table);
            tokio::spawn(async move {
                handle_session(global, table, stream, addr, f(l)).await;
            });
        }
    }
}

#[derive(Debug)]
pub struct Global {
    pub as_number: u32,
    pub id: Ipv4Addr,
    pub peers: HashMap<IpAddr, Peer>,
    pub init: mpsc::Sender<()>,
}
impl Global {
    pub fn new(init: mpsc::Sender<()>) -> Global {
        Global {
            as_number: 0,
            id: Ipv4Addr::new(0, 0, 0, 0),
            peers: HashMap::new(),
            init: init,
        }
    }
}

async fn handle_session(
    global: Arc<Mutex<Global>>,
    table: Arc<Mutex<Table>>,
    stream: TcpStream,
    addr: IpAddr,
    local_addr: IpAddr,
) {
    {
        let mut g = global.lock().await;
        let peer = Peer::new(addr);
        g.peers.insert(addr, peer);
        for (key, value) in &g.peers {
            println!("Peers Key -> {} Peers Val ->  {}", key, value);
        }
    }
    println!("Waiting for peer messages..");
    // LinesCodec is a simple codec that looks for /n breaks
    let mut session = Framed::new(stream, LinesCodec::new());
    loop {
        match session.next().await {
            Some(Ok(msg)) => {
                println!("{}: {}", &addr, msg);
                let mut t = table.lock().await;
                t.master.push(msg.to_string());
                println!("Contents of Table ----->  {:?}", t.master);
                t.routes.insert(addr, msg);
                for (peer, route) in &t.routes {
                    println!("Table Peer -> {} Peer Routes ->  {}", peer, route);
                }

                // TODO Add a Nested Vector for Routes


            }
            Some(Err(_)) => {
                println!("Connection error from {}", &addr);
            }
            None => {
                println!("Lost connection with {}", &addr);

                let mut g = global.lock().await;
                g.peers.remove(&addr);
                for (key, value) in &g.peers {
                    println!("Peers Key -> {} Peers Val ->  {}", key, value);
                }
                break;
            }
        }
    }
    println!("disconnected {}", addr);
}

#[derive(Debug)]
pub struct Peer {
    pub address: IpAddr,
    pub remote_as: u32,
    pub router_id: Ipv4Addr,
}

// Example implementing Display Debug
impl Display for Peer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Debug Print ---> ({}, {})", self.address, self.router_id)
    }
}

impl Peer {
    fn addr(&self) -> String {
        match self.address {
            IpAddr::V4(_) => self.address.to_string(),
            IpAddr::V6(addr) => match addr.to_ipv4() {
                Some(x) => x.to_string(),
                None => self.address.to_string(),
            },
        }
    }

    pub fn new(address: IpAddr) -> Peer {
        Peer {
            address: address,
            remote_as: 0,
            router_id: Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    pub disable_best_path_selection: bool,
    pub master: Vec<String>,
    pub routes: HashMap<IpAddr, String>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            disable_best_path_selection: false,
            master: Vec::new(),
            routes:  HashMap::new(),
        }
    }
}
