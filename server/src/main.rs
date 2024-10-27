use std::collections::HashMap;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct Peer {
    address: String,
}

#[derive(Debug)]
struct P2PNetwork {
    peers: HashMap<i32, Peer>, // Keyed by peer ID or address
}

impl P2PNetwork {
    fn new() -> Self {
        P2PNetwork {
            peers: HashMap::new(),
        }
    }

    fn add_peer(&mut self, peer_id: i32, address: String) {
        self.peers.insert(peer_id, Peer { address });
    }

    async fn send_to_peer(&self, peer_id: &i32, data: &[u8]) -> io::Result<()> {
        if let Some(peer) = self.peers.get(peer_id) {
            connect_to_peer(&peer.address, data).await?;
        } else {
            println!("Peer not found");
        }
        Ok(())
    }
}

// Function to connect to another peer and send a message or file
async fn connect_to_peer(address: &str, data: &[u8]) -> io::Result<()> {
    let mut stream = TcpStream::connect(address).await?;
    println!("Connected to peer at {}", address);

    // Send the data to the peer
    stream.write_all(data).await?;
    println!("Data sent to peer");

    // Optionally, read a response
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!(
        "Received response from peer: {}",
        String::from_utf8_lossy(&buffer[..n])
    );

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080"; // Server address
    let listener = TcpListener::bind(addr).await?;
    let mut network = P2PNetwork::new();
    let mut count: i32 = 0;
    println!("Server listening on {}", addr);

    loop {
        let (_, addr) = listener.accept().await?;
        println!("New Peer {}", addr);
        network.add_peer(count, addr.to_string());
        count += 1;

        if count >= 2 {
            let data = network.peers.get(&count).unwrap().address.as_bytes();
            network.send_to_peer(&(count - 1), data).await?;

            let data = network.peers.get(&(count - 1)).unwrap().address.as_bytes();
            network.send_to_peer(&count, data).await?;
        }
    }
}
