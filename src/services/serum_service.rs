use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_account_decoder::UiAccountEncoding;
use solana_sdk::pubkey::Pubkey;
use serum_dex::state::OpenOrders;
use std::str::FromStr;
use anyhow::Result;
// use crate::Value;
use serde_json::Value;

use reqwest::Client;
use serde_json::json;
use hex;
use crate::models::PoolState; 
use anchor_lang::AccountDeserialize;


pub async fn fetch_open_orders() -> Result<Value> {
    // Helius RPC 节点 + API Key
    let helius_rpc_url = "https://mainnet.helius-rpc.com/?api-key=375fedf5-7461-4e1b-9571-b2dbc5919d9e";
    let client = RpcClient::new(helius_rpc_url.to_string());

    // Serum Market 地址
    let market_pubkey = Pubkey::from_str("3Lzzft9ahF3Yr29eCWQ2L7y2j5MTaMzE3ax9szaMFnNx")?;

    // OpenOrders 程序 ID (Serum DEX v3)
    let open_orders_program_id = Pubkey::from_str("9xQeWvG816bUx9EP6jL1eXJc4f3UbqY4mMKDBj2u2Wv")?;

    let filters = Some(vec![
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            13,
            &market_pubkey.to_bytes(), // ✅ 传入字节数组
        )),
    ]);
    // ✅ 再定义 config
    let config = RpcProgramAccountsConfig {
        filters,
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            ..Default::default()
        },
        with_context: None,
        sort_results: None,
    };
    
    // 获取 Program Accounts
    let accounts = client
    .get_program_accounts_with_config(&open_orders_program_id, config)
    .await?;

    println!("✅ Found {} OpenOrders accounts", accounts.len());

    let pubkey = Pubkey::from_str("2AXXcN6oN9bBT5owwmTH53C7QHUXvhLeu718Kqt8rvY2")?;
    let get_balance = client.get_balance(&pubkey).await?;
    println!("✅ Found balance: {}", get_balance);

    let pubkey = Pubkey::from_str("2AXXcN6oN9bBT5owwmTH53C7QHUXvhLeu718Kqt8rvY2")?;
    let account_info = client.get_account(&pubkey).await?;
    println!("✅ Account lamports: {}", account_info.lamports);
    println!("✅ Account owner: {}", account_info.owner);
    println!("✅ Data length: {}", account_info.data.len());
    println!("✅ Full account info: {:#?}", account_info);
    println!("data (base64): {}", base64::encode(&account_info.data));
println!("data (hex): {}", hex::encode(&account_info.data));
let expected = std::mem::size_of::<crate::models::pool_state::PoolState>();
println!("== expected PoolState size = {}", expected);

let data = &account_info.data[8..]; // skip Anchor discriminator

if let Some(pool_state) = parse_pool_state(&data) {
    let tick_current = pool_state.tick_current;
    let liquidity = pool_state.liquidity;
    let status = pool_state.status;
    println!("✅ Pool owner: {:?}", pool_state.owner);
    println!("✅ Token A mint: {:?}", pool_state.token_mint_0);
    println!("✅ Token B mint: {:?}", pool_state.token_mint_1);
    println!("✅ Liquidity: {}", liquidity);
    println!("✅ Current tick: {}", tick_current);
    println!("✅ Reward0 mint: {:?}", pool_state.reward_infos[0].token_mint);
} else {
    println!("❌ Failed to parse PoolState");
}

    // 遍历解析
    for (_pubkey, account) in accounts {
        // serum_dex 没有 `unpack()`，要手动用 bytemuck 解析
        if let Ok(open_orders) = bytemuck::try_from_bytes::<OpenOrders>(&account.data) {
            let orders = open_orders.orders; // ✅ 复制出来，避免引用未对齐的内存
            let has_orders = orders.iter().any(|&x| x != 0);
            if has_orders {
                // ✅ 正确写法
                let owner = open_orders.owner; // 这里会自动复制出值
                println!("📦 User wallet: {:?}", owner);
            }
        }
    }

    Ok(().into())
}


pub async fn fetch_serum_info() -> Result<Value> {
    let coinmarketcap_rpc_url = "https://dapi.coinmarketcap.com/dex/v1/tokens/trending/list";
    let client = Client::new();
   // 建立請求的 JSON body
   let body = json!({
    "nextPageIndex": "",
    "interval": "24h",
    "pageSize": 100,
    "platformIds": "16"
});
let mut result = json!({}); 
  // 發送 POST 請求
  let response = client
  .post(coinmarketcap_rpc_url)
  .json(&body)
  .send()
  .await?;
      // 檢查是否成功
      if response.status().is_success() {
        let body  = response.text().await?;
        result = json!({
        "transaction": body
    });
        println!("✅ 成功回傳資料：\n{}", body);
    } else {
        println!("❌ 請求失敗，狀態碼: {}", response.status());
    }

    Ok(result)
}

pub async fn fetch_serum_pool_list(addr: &str) -> Result<Value> {
    let coinmarketcap_rpc_url = format!(
        "https://dapi.coinmarketcap.com/dex/v1/token/pools?platform=solana&address={}",
        addr
    );
    let client = Client::new();
 
let mut result = json!({}); 
  // 發送 POST 請求
  let response = client
  .get(coinmarketcap_rpc_url)
  .send()
  .await?;
      // 檢查是否成功
      if response.status().is_success() {
        let body  = response.text().await?;
        result = json!({
        "transaction": body
    });
        println!("✅ 成功回傳資料：\n{}", body);
    } else {
        println!("❌ 請求失敗，狀態碼: {}", response.status());
    }

    Ok(result)
}

pub fn parse_pool_state(data: &[u8]) -> Option<PoolState> {
    if data.len() < std::mem::size_of::<PoolState>() {
        return None;
    }
    bytemuck::try_from_bytes::<PoolState>(data).ok().copied()
}