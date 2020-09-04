mod kcp_server;
mod kcp;


use std::error::Error;
use futures::executor::block_on;
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

    let  kcp = kcp_server::KcpListener::<()>::new("0.0.0.0:5555", config).await?;
    kcp.set_buff_input(|peer, data| {
        // block_on(async move{
        //     let mut lock=  peer.lock().await;
        //     let mut peer=lock.get().unwrap();
        //     peer.send(&data).await;
        // });

        Ok(())
    }).await;

    kcp.start().await?;

    Ok(())
}
