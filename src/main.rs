use clap::Parser;
use options::Options;

use std::sync::Arc;
use std::{net::SocketAddr, time::Duration};
use tokio::{net::UdpSocket, sync::mpsc, time::timeout};
mod options;

#[tokio::main]
async fn main() {
    let options = Options::parse();

    tokio::spawn(run_server(options)).await.unwrap();
}

async fn run_server(options: Options) -> ! {
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(200);

    tokio::spawn(async move {
        let mut idx = 0;
        while let Some(message) = rx.recv().await {
            let cnt = message.len() / 3;
            let mut led_colors = Vec::new();
            for i in 0..cnt {
                let color = &message[(i * 3)..=((i * 3) + 2)];
                led_colors.push(format!("{},{:?}", idx, color).to_string());
                idx = (idx + 1) % options.leds;
            }
            let payload = format!(
                "{{\"on\":\"on\",\"bri\":200,\"seg\":{{\"i\":[{}]}}}}",
                &led_colors.join(",")
            );
            println!("Payload: {}", payload);
            if let Err(e) = reqwest::Client::new()
                .post(options.wled_api.clone())
                .timeout(Duration::from_millis(500))
                .body(payload)
                .header("Content-Type", "application/json")
                .send()
                .await
            {
                println!("Error {:?}", e);
            }
        }
    });

    match UdpSocket::bind(options.bind).await {
        Ok(s) => {
            let socket = Arc::new(s);
            loop {
                let mut buf = [0; 512];
                match socket.recv_from(&mut buf).await {
                    Ok((len, src)) => {
                        let req = buf[..len].to_vec();
                        if let Err(e) = tx.send(req.clone()).await {
                            println!("Error add to queue: {:?}", e);
                        }
                        let socket_copy = Arc::clone(&socket);
                        tokio::spawn(async move {
                            query(req, src, socket_copy, options.upstream).await
                        });
                    }
                    Err(e) => {
                        println!("Error receive: {:?}", e);
                        continue;
                    }
                };
            }
        }
        Err(e) => {
            println!("could not bind socket: {:?}", e);
            std::process::exit(1);
        }
    }
}

async fn query(req: Vec<u8>, src: SocketAddr, socket: Arc<UdpSocket>, upstream_addr: SocketAddr) {
    match UdpSocket::bind(("0.0.0.0", 0)).await {
        Ok(upstream) => {
            match timeout(Duration::from_secs(2), async {
                if let Err(e) = upstream.send_to(&req, upstream_addr).await {
                    println!("Error upstream: {:?}", e);
                    return Err(());
                }
                let mut res = [0; 512];
                match upstream.recv(&mut res).await {
                    Ok(len) => Ok(res[..len].to_vec()),
                    Err(e) => {
                        println!("Error receive upstream: {:?}", e);
                        Err(())
                    }
                }
            })
            .into_inner()
            .await
            {
                Ok(data) => _ = socket.send_to(&data, src).await,
                Err(e) => println!("Error query: {:?}", e),
            };
        }
        Err(e) => println!("Error upstream socket: {:?}", e),
    }
}
