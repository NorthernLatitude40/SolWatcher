use anyhow::Context;
use reqwest::Client;
use std::fmt;
use crate::models;
use models::Pool::Pool;
use serde_json::Value;
use serde_json::json;

pub async fn fetch_plmint_accounts(addr: &str) ->  anyhow::Result<Value> {
    // 指定你要查的代幣 mint
    let _target_mint = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R";

   // 创建 HTTP 客户端
   let client = Client::new();

   let body = json!({
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getTokenLargestAccounts",
    "params": [
        addr
    ]
  });

   // 发送 GET 请求获取池 ID 列表
   let res = client
       .get("https://mainnet.helius-rpc.com/?api-key=375fedf5-7461-4e1b-9571-b2dbc5919d9e")
       .json(&body)  
       .send().await?;


       
    let mut result = json!({}); 
   // 检查响应状态码
   if res.status().is_success() {
       // 打印响应体
       let body = res.text().await?;
        result = json!({
        "transaction": body
        });
       println!("Response: {}", body);
       
   } else {
       // 打印错误信息
       eprintln!("Error: {}", res.status());
   }
   Ok(result)
}
