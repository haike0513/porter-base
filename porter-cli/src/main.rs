use std::str::FromStr;

use anyhow::Result;
use iroh::{protocol::Router, Endpoint, SecretKey, Watcher};
use iroh_ping::{ALPN as PingALPN, Ping};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let mut rng = rand::rngs::OsRng;
    // let sk = SecretKey::generate(&mut rng);
    // let sk_format = hex::encode(sk.secret().as_bytes());
    // log::info!("{:#?}", sk_format);
    let decode_bytes = hex::decode("69c5d22a918631f1a2daa9ff0b8b764b83c1c6c9222288235772a60bb01df975")?;
    let secret_key = SecretKey::try_from(decode_bytes.as_slice())?;
        log::info!("Start");
    let recv_ep = Endpoint::builder()
        .discovery_local_network()
        .discovery_dht()
        .discovery_n0()
        .secret_key(secret_key.to_owned())
        .bind()
        .await?;
    let recv_router = Router::builder(recv_ep)
        .accept(PingALPN, Ping::new())
        .spawn();
    let addr = recv_router.endpoint().node_addr().get().unwrap();


    let decode_bytes = hex::decode("4f35d1ee7a95c9b8d866a4f2fb4a1d5973670373b2ea7de3f99bbf6ed3883485")?;
    let secret_key = SecretKey::try_from(decode_bytes.as_slice())?;
    // create a send side & send a ping
    let send_ep = Endpoint::builder()
        .discovery_local_network()
        .discovery_dht()
        .discovery_n0()
        .secret_key(secret_key)
        .bind()
        .await?;
    let send_pinger = Ping::new();
    let rtt = send_pinger.ping(&send_ep, addr).await?;
    println!("ping took: {rtt:?} to complete");
    Ok(())
}
