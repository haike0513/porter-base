use anyhow::Result;
use iroh::{protocol::Router, Endpoint, Watcher};
use iroh_ping::{ALPN as PingALPN, Ping};
#[tokio::main]
async fn main() -> Result<()> {
    let recv_ep = Endpoint::builder().discovery_n0().bind().await?;
    let recv_router = Router::builder(recv_ep)
        .accept(PingALPN, Ping::new())
        .spawn();
    let addr = recv_router.endpoint().node_addr().get().unwrap();

    // create a send side & send a ping
    let send_ep = Endpoint::builder().discovery_n0().bind().await?;
    let send_pinger = Ping::new();
    let rtt = send_pinger.ping(&send_ep, addr).await?;
    println!("ping took: {rtt:?} to complete");
    Ok(())
}
