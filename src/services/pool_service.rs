use anyhow::Context;
use reqwest::Client;
use std::fmt;
use crate::models;
use models::Pool::Pool;
use serde_json::Value;
use serde_json::json;

pub async fn fetch_pools_info() ->  anyhow::Result<Value> {
    impl fmt::Display for Pool {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Pool {{ lp_mint: {}, totalLiquidity: {:?} }}", self.lp_mint, self.total_liquidity)
        }
    }
    // 指定你要查的代幣 mint
    let _target_mint = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R";

   // 创建 HTTP 客户端
   let client = Client::new();

   // 发送 GET 请求获取池 ID 列表
   let res = client
       .get("https://api-v3.raydium.io/pools/info/list-v2?poolType=Standard&size=5")
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

pub async fn fetch_pool_ids(id_list: Vec<String>) ->  anyhow::Result<Value> {


   // 创建 HTTP 客户端
   let client = Client::new();
   let ids_str_by_comma = id_list.join(",");

   // 拼接 URL
   let url = format!("https://api-v3.raydium.io/pools/info/ids?ids={}", ids_str_by_comma);

   // 发送 GET 请求获取池 ID 列表
   let res = client.get(&url).send().await?;


       
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

pub async fn fetch_solbalance_id(id: String) ->  anyhow::Result<Value> {


    // 创建 HTTP 客户端
    let client = Client::new();
 
    // 拼接 URL
    let url = format!("https://api.mainnet-beta.solana.com");

    // 构造 JSON body
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": ["2AXXcN6oN9bBT5owwmTH53C7QHUXvhLeu718Kqt8rvY2",   {
            "commitment": "finalized"
          }] // 替换成目标钱包地址
    });
 
    let res = client
    .post(url)
    .json(&body)   // 这里将 JSON body 放入请求
    .send()
    .await?;
 
 
        
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