use std::path::Path;

use iroh::{Endpoint, SecretKey, Watcher};
use iroh_base::ticket::NodeTicket;
use iroh_blobs::{
    api::downloader::{self, Shuffled},
    store::fs::FsStore,
};
use n0_future::StreamExt;
pub async fn run() -> anyhow::Result<()> {
    let root = std::path::absolute("./")?;
    let store = FsStore::load(&root.join(".porter")).await?;
    let tag = store.add_path(&root.join("Cargo.toml")).await?;
    log::info!("added Cargo.toml with tag {}", tag.hash_and_format().hash);
    let decode_bytes =
        hex::decode("4f35d1ee7a95c9b8d866a4f2fb4a1d5973670373b2ea7de3f99bbf6ed3883485")?;
    let secret_key = SecretKey::try_from(decode_bytes.as_slice())?;
    // create a send side & send a ping
    let endpoint = Endpoint::builder()
        .discovery_local_network()
        .discovery_dht()
        .discovery_n0()
        .secret_key(secret_key)
        .bind()
        .await?;
    let blobs = iroh_blobs::BlobsProtocol::new(&store, endpoint.clone(), None);
    let router = iroh::protocol::Router::builder(endpoint.clone())
        .accept(iroh_blobs::ALPN, blobs)
        .spawn();
    let addr = router.endpoint().node_addr().initialized().await;
    let ticket = NodeTicket::from(addr.clone());
    log::info!("Node address: {addr:?}");
    log::info!("ticket:\n{ticket}");
    let downloader = store.downloader(&endpoint);
    let nodes = vec![addr.node_id.clone()];
    let mut progress = downloader
        .download(tag.hash_and_format(), Shuffled::new(nodes))
        .stream()
        .await?;
    while let Some(event) = progress.next().await {
        log::info!("Progress: {:?}", event);
    }
    log::info!("Download complete");
    store.export(tag.hash, root.join(".export")).await?;
    store.dump().await?;
    Ok(())
}
