use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use byteorder::{LittleEndian, ReadBytesExt};
use serde_json::json;
use std::io::Cursor;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

//账户关联：
// Instruction 数据只存参数
// 实际操作的 token / pool / user 都在 transaction 的 accountKeys 中
fn main() -> anyhow::Result<()> {
    let ray_log = std::env::args().nth(2).unwrap_or_else(|| {
        // 預設示範字串（你也可以從命令列傳入）
        "AzWn51r/AAAAAAAAAAAAAAACAAAAAAAAADWn51r/AAAA9l8fZ/4yNACXGFdjJgAAAPVOuwAAAAAA".to_string()
    });

    let data_bytes = general_purpose::STANDARD.decode(&ray_log)?;
    let mut rdr = Cursor::new(&data_bytes);

    // 1 byte tag (u8)
    let tag = rdr.read_u8().context("read tag failed")?;
    // two u64 little-endian (amount_in, minimum_amount_out)
    let amount_in = rdr
        .read_u64::<LittleEndian>()
        .context("read amount_in failed")?;
    let minimum_amount_out = rdr
        .read_u64::<LittleEndian>()
        .context("read minimum_amount_out failed")?;

    // 其餘 bytes（hex 列表）
    let mut remaining = Vec::new();
    while let Ok(b) = rdr.read_u8() {
        remaining.push(format!("{:02X}", b));
    }

    let result = json!({
        "tag": tag,
        "instruction": match tag {
            3 => "SwapBaseIn",
            4 => "SwapBaseOut",
            2 => "SomeOther", // 視版本而定
            _ => "Unknown"
        },
        "amount_in": amount_in,
        "minimum_amount_out": minimum_amount_out,
        "remaining_hex": remaining.join(" ")
    });

    println!("{}", serde_json::to_string_pretty(&result)?);
    // ===========================================================
    let sig = std::env::args()
    .nth(1)
    .unwrap_or_else(|| {
        // 預設示範字串（你也可以從命令列傳入）
        "5f8nKzSViJ7xDzL2j5nz8pFks9NuiWgd3eoX3teVJHNcq7kzSv3PY5Gknuzswu4eTcuNsYGGiW7oF7KansEpGNH6".to_string()
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
        let client = reqwest::blocking::Client::new();
        let resp: Value = client
            .post(RPC_URL)
            .json(&payload)
            .send()?
            .json()
            .context("RPC 回應解析失敗")?;
    let tx = resp["result"].clone();
    let program_ids: Vec<String> = tx["transaction"]["message"]["accountKeys"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v["pubkey"].as_str().map(|s| s.to_string()))
        .collect();
  
    let raydium_v4_program = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")?;
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
        return Ok(());
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
    Ok(())
}
