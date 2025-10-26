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
