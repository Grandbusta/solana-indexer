use solana_client::{
    rpc_client::RpcClient,
    // pubsub_client::PubsubClient,
    nonblocking::pubsub_client::PubsubClient as NonBlockingPubsubClient,
};

pub enum SolanaConnection {
    Rpc(RpcClient),
    WebSocket(NonBlockingPubsubClient),
}

pub async fn establish_connection(conn_type: &str) -> Result<SolanaConnection, String> {
    match conn_type {
        "socket" => {
            let ws_url = std::env::var("SOLANA_WS_URL")
                .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string());
            println!("Establishing WebSocket connection to Solana {}", ws_url);
            
            NonBlockingPubsubClient::new(&ws_url)
                .await
                .map(SolanaConnection::WebSocket)
                .map_err(|e| format!("Failed to connect to WebSocket: {}", e))
        },
        _ => {
            let rpc_url = std::env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
            println!("Establishing RPC connection to Solana {}", rpc_url);
            
            Ok(SolanaConnection::Rpc(RpcClient::new(rpc_url)))
        }
    }
}