use std::{collections::HashSet, io, string};
use tokio::net::UdpSocket;

pub async fn server() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    log::info!("Server listening on 0.0.0.0:8080");

    let mut seen = HashSet::<u64>::new();
    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let data = &buf[..len];

        if data.len() < 8 {
            log::error!("Packet received is too small from {}", addr);
        }

        let seq = u64::from_be_bytes(data[..8].try_into().unwrap());
        let payload = &data[8..];

        if seen.contains(&seq) {
            log::info!("Duplicate packet {} from {}", seq, addr);
        } else {
            seen.insert(seq);
            println!("Hashset {:?}", seen);
            log::info!(
                "Received {} from {}: {}",
                seq,
                addr,
                String::from_utf8_lossy(payload)
            );
        }

        let ack = format!("ACK {}", seq);
        socket.send_to(ack.as_bytes(), addr).await?;
    }
}

pub async fn client() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let server = "0.0.0.0:8080";

    for seq in 1u64..=5 {
        let payload = format!("mesh-core is live {}", seq);
        let mut packet = Vec::with_capacity(8 + payload.len());

        packet.extend_from_slice(&seq.to_be_bytes());
        packet.extend_from_slice(&payload.as_bytes());

        socket.send_to(&packet, server).await?;
        log::info!("Send sequence {}", seq);

        let mut ack_buf = [0u8; 64];
        if let Ok((len, addr)) = socket.recv_from(&mut ack_buf).await {
            log::info!(
                "Got ACK: {} from {}",
                String::from_utf8_lossy(&ack_buf[..len]),
                addr
            );
        }
    }
    Ok(())
}
