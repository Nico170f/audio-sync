use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8080").await?);
    println!("Host listening on 0.0.0.0:8080");

    let clients = Arc::new(Mutex::new(HashSet::<SocketAddr>::new()));
    let socket_recv = Arc::clone(&socket);
    let clients_recv = Arc::clone(&clients);

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            if let Ok((len, addr)) = socket_recv.recv_from(&mut buf).await {
                println!("Received {} bytes from {:?}", len, addr);
                clients_recv.lock().unwrap().insert(addr);
            }
        }
    });

    let mut interval = time::interval(Duration::from_millis(50));
    loop {
        interval.tick().await;
        let data = b"Hello from host!";
        for client in clients.lock().unwrap().iter() {
            let _ = socket.send_to(data, client).await;
        }
    }
}
