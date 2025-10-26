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