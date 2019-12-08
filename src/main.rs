//! A proxy that forwards data to another server and forwards that server's
//! responses back to clients.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! You can showcase this by running this in one terminal:
//!
//!     cargo run --example proxy
//!
//! This in another terminal
//!
//!     cargo run --example echo
//!
//! And finally this in another terminal
//!
//!     cargo run --example connect 127.0.0.1:8081
//!
//! This final terminal will connect to our proxy, which will in turn connect to
//! the echo server, and you'll be able to see data flowing between them.

#![warn(rust_2018_idioms)]

mod proxy;

use tokio::runtime;
use proxy::ProxyServer;
use std::env;

fn main() {
    let listen_addr = env::args().nth(1).unwrap_or("127.0.0.1:8081".to_string());

    let mut threaded_rt: runtime::Runtime = runtime::Builder::new().threaded_scheduler().build().unwrap();
    threaded_rt.block_on(async {
        let mut proxy_server = ProxyServer::new(&listen_addr).await.unwrap();
        
        println!("proxy server will start on {}", listen_addr);
        let result = proxy_server.start().await;
        match result {
            Ok(_) => println!("proxy exit successfully!"),
            Err(err) => println!("proxy exit with error: {}", err),
        }
    });
}