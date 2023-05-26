mod mempool;


const WSS_URL: &str = "wss://your_link";


#[tokio::main]
async fn main() -> web3::Result<()> {
    let websocket = web3::transports::WebSocket::new(WSS_URL).await?;
    let web3s = web3::Web3::new(websocket);

    mempool::mempool(web3s).await;

    Ok(())
}