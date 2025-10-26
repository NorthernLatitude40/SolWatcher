use anyhow::Context;
use reqwest::Client;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use serde_json::json;

pub async fn fetch_transaction() ->  anyhow::Result<Value> {
    let sig = std::env::args()
    .nth(1)
    .unwrap_or_else(|| {
        // 預設示範字串（你也可以從命令列傳入）
        "3hwnH5wBNHVhZ9DxkCkVLJGvWs1fJtEkNbPRBKGs29LwNUBb872cuetQPiD7HgraWrNv9S8DK3GHgpPTvuSt3wyV".to_string()
    });
    
      // 1. 準備 JSON-RPC 請求
      let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            sig,
            {
                "encoding": "jsonParsed",
                "maxSupportedTransactionVersion": 0
            }
        ]
    });
        // 2. 發送 RPC 請求
        const RPC_URL: &str = "https://api.mainnet-beta.solana.com";
        let client = Client::new();
        let resp: Value = client
            .post(RPC_URL)
            .json(&payload)
            .send().await?
            .json()
            .await
            .context("RPC 回應解析失敗")?;
    let tx = resp["result"].clone();
    let program_ids: Vec<String> = tx["transaction"]["message"]["accountKeys"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v["pubkey"].as_str().map(|s| s.to_string()))
        .collect();
    
    let raydium_v4_program = Pubkey::from_str("4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R")?;
    println!("raydium_v4_programraydium_v4_programraydium_v4_program pubkey {}", raydium_v4_program);
    let _is_raydium = program_ids.iter().any(|id| {
        if let Ok(pubkey) = Pubkey::from_str(id) {
            pubkey == raydium_v4_program
        } else {
            false
        }
    });
    
    // 3. 抽取必要資訊
    if tx.is_null() {
        println!("❌ 沒找到交易，可能 signature 錯或未確認。");
        return Ok(().into());
    }
    
    
    let is_raydium = program_ids.iter().any(|id| {
        if let Ok(pubkey) = Pubkey::from_str(id) {
            pubkey == raydium_v4_program
        } else {
            false
        }
    });
    println!("🔍 Program IDs: {:?}", program_ids);
    println!("✅ 是否 Raydium AMM v4: {}", is_raydium);
    
    // 4. 查 log 確認指令
    if let Some(logs) = tx["meta"]["logMessages"].as_array() {
        for line in logs {
            if let Some(l) = line.as_str() {
                if l.contains("Instruction:") {
                    println!("📜 {}", l);
                }
                if l.contains("src_token_amount") || l.contains("dst_token_amount") {
                    println!("💰 {}", l);
                }
            }
        }
    }
    
    // 5. pre/post token balance 變化
    println!("\n📊 Token balance 變化：");
    if let (Some(pre), Some(post)) =
        (tx["meta"]["preTokenBalances"].as_array(), tx["meta"]["postTokenBalances"].as_array())
    {
        for (_i, (a, b)) in pre.iter().zip(post).enumerate() {
            let owner = a["owner"].as_str().unwrap_or("");
            let mint = a["mint"].as_str().unwrap_or("");
            let pre_bal = a["uiTokenAmount"]["uiAmountString"].as_str().unwrap_or("?");
            let post_bal = b["uiTokenAmount"]["uiAmountString"].as_str().unwrap_or("?");
            if pre_bal != post_bal {
                println!(
                    "帳戶 {} (mint={})：{} → {}",
                    owner, mint, pre_bal, post_bal
                );
            }
        }
    } else {
        println!("無 pre/post token balance 資訊");
    }
    
    let result = json!({
        "transaction": tx
    });
    Ok(result)
}


