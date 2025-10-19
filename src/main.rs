use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionLogsFilter;
use solana_sdk::pubkey::Pubkey;
use bincode::deserialize;
use serde_json::json;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 广播通道，用于推送 JSON 到 WebSocket 客户端
    let (tx, _rx) = broadcast::channel(100);
    let tx_ws = tx.clone();

    // Warp WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx_ws.clone();
            ws.on_upgrade(move |socket| handle_ws(socket, tx))
        });

    tokio::spawn(async move {
        warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
    });

    // Solana RPC + WebSocket
    let raydium_program_id = Pubkey::from_str("RaydiumAMMProgramID")?;
    let rpc_client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com"));

    let (mut client, mut receiver) = PubsubClient::logs_subscribe(
        "wss://api.mainnet-beta.solana.com",
        RpcTransactionLogsFilter::Mentions(vec![raydium_program_id]),
    )
    .await?;

    println!("Subscribed to Raydium AMM logs...");

    while let Some(logs) = receiver.next().await {
        let logs = logs?;

        // 简单示例：找到 amm_id
        if let Some(amm_id_str) = logs.value.logs.iter().find(|line| line.contains("amm_id")) {
            // 提取 amm_id 地址
            let amm_id = extract_pubkey(amm_id_str);
            let account = rpc_client.get_account(&amm_id).await?;
            let amm_info: AmmInfo = deserialize(&account.data)?;

            // 生成 JSON
            let json_data = json!(amm_info);

            // 推送到 WebSocket 客户端
            let _ = tx.send(json_data.to_string());
        }
    }

    Ok(())
}

// WebSocket 处理函数
async fn handle_ws(ws: warp::ws::WebSocket, tx: broadcast::Sender<String>) {
    let (mut ws_tx, mut _ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let _ = ws_tx.send(warp::ws::Message::text(msg)).await;
        }
    });
}

// 简单提取 Pubkey
fn extract_pubkey(log_line: &str) -> Pubkey {
    let parts: Vec<&str> = log_line.split_whitespace().collect();
    for part in parts {
        if part.len() == 44 || part.len() == 43 { // Solana pubkey 长度
            if let Ok(pubkey) = Pubkey::from_str(part) {
                return pubkey;
            }
        }
    }
    panic!("No pubkey found in log_line: {}", log_line);
}
