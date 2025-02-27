use dotenvy::dotenv;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::transaction;
use solana_transaction_status::{
    EncodedTransaction, TransactionDetails, UiMessage, UiTransactionEncoding,
    UiTransactionStatusMeta,
};
mod config;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    let filter = RpcBlockSubscribeFilter::All;
    let config = RpcBlockSubscribeConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        show_rewards: Some(false),
        max_supported_transaction_version: Some(0),
    };

    let url = std::env::var("SOLANA_WS_URL")
        .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string());

    let subscription_result = PubsubClient::block_subscribe(&url, filter, Some(config));

    match subscription_result {
        Ok((_tx_confirmed_block, rx_confirmed_block)) => {
            // loop {
            match rx_confirmed_block.recv() {
                Ok(response) => {
                    let block = response.value.block.unwrap();
                    println!(
                        "block: {:?}, time: {:?}",
                        block.block_height, block.block_time
                    );
                    let txns = block.transactions.unwrap_or_else(|| vec![]);
                    for txn in txns {
                        println!("txn: {:?}", txn.transaction);


                        let tx_meta = txn.meta.unwrap();
                        println!("txn meta: {:?}", tx_meta);

                        // store: status, from, to, amount, block_height, block_time, signature, fee
                        match txn.transaction {
                            EncodedTransaction::Json(data) => {
                                let signatures = data.signatures;
                                let message = data.message;

                                let signature = signatures[0].to_string();

                                let pre_balances = tx_meta.pre_balances;
                                let post_balances = tx_meta.post_balances;

                                // let amount = pre_balances[0] - post_balances[0] - tx_meta.fee;

                                println!(
                                    "prebalance {:?}, postbalance: {:?}, fee: {:?}",
                                    pre_balances, post_balances, tx_meta.fee
                                );

                                match message {
                                    UiMessage::Parsed(msg) => {
                                        // let from_address = &msg.account_keys[0].pubkey;
                                        // let to_address = &msg.account_keys[1].pubkey;
                                        // println!(
                                        //     "from: {:?}, to: {:?}, amount: {:?}",
                                        //     from_address, to_address, amount
                                        // );
                                    }
                                    UiMessage::Raw(msg) => {
                                        // let from_address = &msg.account_keys[0];
                                        // let to_address = &msg.account_keys[1];
                                        // println!(
                                        //     "from: {:?}, to: {:?}, amount: {:?}",
                                        //     from_address, to_address, amount
                                        // );
                                    }
                                }
                            }
                            EncodedTransaction::Binary(_, _) => {
                                println!("  Binary transaction (skipping)");
                            }
                            EncodedTransaction::LegacyBinary(_) => {
                                println!("  Legacy binary transaction (skipping)");
                            }
                            EncodedTransaction::Accounts(_) => {
                                println!("  Accounts (skipping)");
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("block subscription error: {:?}", e)
                    // break;
                }
            }
            // }
        }
        Err(e) => {
            eprintln!("block subscription error: {:?}", e)
        }
    }

    sleep(Duration::from_secs(1)).await;
}
