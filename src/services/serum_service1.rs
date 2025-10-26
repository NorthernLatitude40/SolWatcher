use anyhow::Result;
use serde_json::json;
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_sdk::pubkey::Pubkey;
use serum_dex::state::OpenOrders;
use spl_token::id as token_program_id;
use std::str::FromStr;
use bytemuck::try_from_bytes;use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_request::TokenAccountsFilter;

pub fn fetch_open_orders() -> Result<serde_json::Value> {
    let mut result_list: Vec<Value>  = vec![];

    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    let version = client.get_version().unwrap();
    println!("Connected to node: {:?}", version);
    // Market / Serum program / LP mint
    let market_pubkey = Pubkey::from_str("3Lzzft9ahF3Yr29eCWQ2L7y2j5MTaMzE3ax9szaMFnNx")?;
    let serum_program_id = Pubkey::from_str("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")?;
    let lp_mint = Pubkey::from_str("6pPGqMpQYW98NUfHvvKZtXzXLpBqDubYffJ3STfMKuT1")?;
    // 2️⃣ 已知 LP 用户列表（示例）
    let users = vec![
        Pubkey::from_str("So11111111111111111111111111111111111111112")?,
        Pubkey::from_str("6d5zHW5B8RkGKd51Lpb9RqFQSqDudr9GJgZ1SgQZpump")?,
    ];
 // 4️⃣ 拉取 Serum 程式下所有帳戶
 println!("------------------------------");
// 使用 Pubkey 代替 Address
let owner_pubkey = Pubkey::from_str("HxQG3hBekCDshAcEwafw5x2fucmN7hJ7ay9yDGMnWeet")?;
let lp_mint = Pubkey::from_str("6pPGqMpQYW98NUfHvvKZtXzXLpBqDubYffJ3STfMKuT1")?;

let accounts = client.get_token_accounts_by_owner(
    &owner_pubkey,
    TokenAccountsFilter::Mint(lp_mint),
)?;
println!("Token accounts: {:?}", accounts);

println!("Total OpenOrders accounts: {}", accounts.len());
println!("------------------------------");

//     for (oo_pubkey, oo_account) in oo_accounts {
//         if let Ok(oo) = try_from_bytes::<OpenOrders>(&oo_account.data) {
//             let oo_market_pubkey  = Pubkey::new_from_array(bytemuck::cast::<[u64; 4], [u8; 32]>(oo.market));
//             if oo_market_pubkey  == market_pubkey {
//                 println!("Owner: {}", owner);
//                 println!("OpenOrders account: {}", oo_pubkey);
//                 result_list.push(json!({
//                     "owner": owner.to_string(),
//                     "open_orders_account": oo_pubkey.to_string(),
//                     "market": oo_market_pubkey.to_string(),
//                 }));
//             }
//         }
//     }
// }


    //===================================================================================

      // 用戶列表
  let token_accounts = vec![
    Pubkey::from_str("HxQG3hBekCDshAcEwafw5x2fucmN7hJ7ay9yDGMnWeet")?,
];
    // // 1️⃣ 查询持有 LP token 的账户
    // let lp_string = lp_mint.to_string();
    // let lp_bytes = lp_string.as_bytes();
    // let filters = vec![RpcFilterType::Memcmp(
    //      Memcmp::new_base58_encoded(0, lp_bytes)
    // )];


    // let token_accounts = client.get_program_accounts_with_config(
    //     &token_program_id(),
    //     RpcProgramAccountsConfig {
    //         filters: Some(filters),
    //         ..Default::default() // 老版本不支持 encoding
    //     },
    // )?;

    // println!("Found {} LP holders", token_accounts.len());

    // let mut result_list = vec![];

    // 2️⃣ 遍历 LP token 所有者
    for account in token_accounts {

        // 查询 Serum OpenOrders
        let owner_string = account.to_string();       // 保存 String
        let owner_bytes = owner_string.as_bytes();         // 切片引用
        let oo_filters = vec![RpcFilterType::Memcmp(
            Memcmp::new_base58_encoded(32, owner_bytes)
        )];
        

        let open_orders_accounts = client.get_program_accounts_with_config(
            &serum_program_id,
            RpcProgramAccountsConfig {
                filters: Some(oo_filters),
                ..Default::default()
            },
        )?;

        for (oo_pubkey, oo_account) in open_orders_accounts {
            if let Ok(oo) = try_from_bytes::<OpenOrders>(&oo_account.data) {
                let oo_market_pubkey =
                    Pubkey::new_from_array(bytemuck::cast::<[u64; 4], [u8; 32]>(oo.market));
                if oo_market_pubkey == market_pubkey {
                    println!("OpenOrders account: {}, market: {}", oo_pubkey, oo_market_pubkey);
                    result_list.push(json!({
                        "owner": owner_pubkey.to_string(),
                        "open_orders_account": oo_pubkey.to_string(),
                        "market": oo_market_pubkey.to_string(),
                    }));
                }
            }
        }
    }

    Ok(json!(result_list))
}

