use tokio::io;
use tokio::net::{TcpListener, TcpStream};

use futures::future::try_join;
use futures::FutureExt;
use std::error::Error;

#[derive(Debug)]
pub struct ProxyServer {
    addr: String,
    listener: TcpListener,
}

impl ProxyServer {

    pub async fn new(addr: &String) -> Result<ProxyServer, Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        Ok(ProxyServer{
            addr: addr.clone(), listener
        })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        while let Ok((inbound, _)) = self.listener.accept().await {
            let transfer = ProxyServer::transfer(inbound, "".to_string()).map(|r| {
                if let Err(e) = r {
                    println!("Failed to transfer; error={}", e);
                }
            });
    
            tokio::spawn(transfer);
        }
    
        Ok(())
    }

    async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
        let mut outbound = TcpStream::connect(proxy_addr).await?;
    
        let (mut ri, mut wi) = inbound.split();
        let (mut ro, mut wo) = outbound.split();
    
        let client_to_server = io::copy(&mut ri, &mut wo);
        let server_to_client = io::copy(&mut ro, &mut wi);
    
        try_join(client_to_server, server_to_client).await?;
    
        Ok(())
    }
}
