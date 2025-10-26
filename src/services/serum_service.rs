use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ProgramAccount {
    pub pubkey: String,
    pub account: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct RpcResponse {
    pub result: Vec<ProgramAccount>,
}

pub async fn fetch_open_orders() -> Result<Value> {
    let api_key = "375fedf5-7461-4e1b-9571-b2dbc5919d9e"; // 替换为你的 Helius API Key
    let client = Client::new();

    // Serum DEX v3 program 地址
    let serum_program = "9xQeWvG816bUx9EP4CAbXg5rxuZHkJoHxh6vZya8Qk8n";

    // 用户公钥
    let user_pubkey = "HxQG3hBekCDshAcEwafw5x2fucmN7hJ7ay9yDGMnWeet";

    // Helius RPC 请求
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getProgramAccounts",
        "params": [
            serum_program,
            {
                "encoding": "jsonParsed",
                "filters": [
                    {
                        "memcmp": {
                            "offset": 32,  // Serum open orders 帐户中 user pubkey 偏移通常是 32
                            "bytes": user_pubkey
                        }
                    }
                ]
            }
        ]
    });

    let res = client
        .post(format!("https://rpc.helius.xyz/?api-key={}", api_key))
        .json(&body)
        .send()
        .await?;

    let rpc_res: RpcResponse = res.json().await?;
        // 格式化返回
        let formatted: Vec<Value> = rpc_res
        .result
        .into_iter()
        .map(|acc| {
            json!({
                "pubkey": acc.pubkey,
                "account": acc.account
            })
        })
        .collect();

    println!("OpenOrders accounts for {}:", user_pubkey);
    // for acc in rpc_res.result {
    //     println!("- {}", acc.pubkey);
    // }

    Ok(json!({
        "user_pubkey": user_pubkey,
        "open_orders": formatted
    }))
}
