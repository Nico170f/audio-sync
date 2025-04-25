use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run --bin client <host_ip>");
        return Ok(());
    }

    let host_ip = &args[1];
    let host_addr = format!("{}:8080", host_ip);

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?); 

    {
        let socket = Arc::clone(&socket);
        let host_addr = host_addr.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let _ = socket.send_to(b"ping", &host_addr).await;
            }
        });
    }

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}: {:?}", len, addr, &buf[..len]);
    }
}
