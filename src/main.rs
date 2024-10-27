use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn receive(mut socket: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = vec![0; 1024];
        let n = socket.read(&mut buffer).await?;
        let receive = String::from_utf8_lossy(&buffer[..n]);
        println!("Message: {}", receive);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let s_addr = "127.0.0.1:8080";
    let stream = TcpStream::connect(s_addr).await?;
    println!("Connected To Server At {}", s_addr);

    let local_addr = stream.local_addr();
    let listener = TcpListener::bind(local_addr.unwrap().to_string()).await?;
    let mut socket: TcpStream;

    loop {
        let (socket_connection, addr) = listener.accept().await?;
        if addr.to_string() == s_addr {
            socket = socket_connection;
            break;
        }
    }

    let mut buffer = vec![0; 1024];
    let n = socket.read(&mut buffer).await?;
    let msg = String::from_utf8_lossy(&buffer[..n]);
    println!("Message: {}", msg.clone());

    let mut peer_stream = TcpStream::connect(msg.clone().to_string()).await?;
    loop {
        let (socket_connection, addr) = listener.accept().await?;
        if addr.to_string() == msg.clone().to_string() {
            socket = socket_connection;
            break;
        }
    }
    tokio::spawn(receive(socket));
    let data = b"Hello, Fellow Peer";
    peer_stream.write_all(data).await?;
    Ok(())
}
