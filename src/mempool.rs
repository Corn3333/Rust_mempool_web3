
use web3::futures::{TryStreamExt};
use web3::types::{TransactionId};



pub async fn mempool(web3: web3::Web3<web3::transports::WebSocket>) {

    let mut pending_transactions = web3.eth_subscribe().subscribe_new_pending_transactions().await.unwrap();

    while let Some(pending_transaction_hash) = pending_transactions.try_next().await.unwrap() {
        let web3 = web3.clone();
        tokio::spawn(async move {
        match web3.eth().transaction(TransactionId::from(pending_transaction_hash)).await {
            Ok(Some(tx)) => {
                println!("Hash: {:?}, status: {:?}", tx.hash, tx.block_hash.is_some());
            }
            Ok(None) => {},
            Err(_) => {}
        }
    });
    }

}