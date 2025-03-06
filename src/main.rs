mod models;
mod indexer;
mod controllers;
mod db;
use std::env;
use db::transaction::{save_transaction, save_accounts};
use sqlx::{pool::PoolOptions, Pool, Postgres};
use axum::{ routing::get, Router };
use dotenvy::dotenv;
use indexer::transaction::get_transactions_from_block;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{
     TransactionDetails, UiTransactionEncoding
};
use tokio::{net::TcpListener, time::{sleep, Duration}};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>
}


async fn listen_to_blocks(
    shared_state: AppState
) {
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
            loop {
            match rx_confirmed_block.recv() {
                Ok(response) => {
                    let block = response.value.block.unwrap();
                    match get_transactions_from_block(block) {
                        Ok(transactions) => {
                            let mut tx = shared_state.db_pool.begin().await.unwrap();
                            let mut success = true;
                            
                            for transaction in transactions {
                                match save_transaction(transaction.clone(), &mut tx).await {
                                    Ok(saved_tx) => {
                                        if let Err(e) = save_accounts(transaction.accounts, saved_tx.id.unwrap(), &mut tx).await {
                                            eprintln!("Error saving accounts: {:?}", e);
                                            success = false;
                                            break;
                                        }
                                        println!("Transaction and accounts saved successfully");
                                    }
                                    Err(e) => {
                                        eprintln!("Error saving transaction: {:?}", e);
                                        success = false;
                                        break;
                                    }
                                }
                            }

                            if success {
                                if let Err(e) = tx.commit().await {
                                    eprintln!("Error committing transaction: {:?}", e);
                                }
                            } else {
                                if let Err(e) = tx.rollback().await {
                                    eprintln!("Error rolling back transaction: {:?}", e);
                                }
                            }
                        }
                        Err(e) => eprintln!("Error processing block: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("block subscription error: {:?}", e)
                }
            }
            }
        }
        Err(e) => {
            eprintln!("block subscription error: {:?}", e)
        }
    }

    sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool: Pool<Postgres> = PoolOptions::new()
        .max_connections(10).connect(&db_url)
        .await.expect("Failed to connect to database");

    let shared_state = AppState {
        db_pool,
    };

    sqlx::migrate!("src/db/migrations").run(&shared_state.db_pool).await.expect("Failed to run migrations");

    let app = Router::new()
        .nest("/transactions", transation_routes())
        .with_state(shared_state.clone());  // Clone here

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Spawn the block listener in a separate task
    tokio::spawn(listen_to_blocks(shared_state));

    axum::serve(listener, app).await.expect("Failed to serve");
}


fn transation_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(controllers::transaction::get_transactions))
}