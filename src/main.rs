#![feature(async_closure)]

mod kcp_server;
mod kcp;

use std::error::Error;
use crate::kcp_server::kcp_config::{KcpConfig, KcpNoDelayConfig};
use env_logger::Builder;
use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{

    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    let mut config = KcpConfig::default();
    config.nodelay = Some(KcpNoDelayConfig::fastest());

    let kcp = kcp_server::KcpListener::<i32,_>::new("0.0.0.0:5555", config).await?;

    kcp.set_buff_input(async move |peer, data| {
        let mut token = peer.token.lock().await;

        if let Some(id) = token.get() {
            *id += 1;
        }

        peer.send(&data).await?;
        peer.flush().await?;
        Ok(())
    }).await;


    kcp.start().await?;

    Ok(())
}
