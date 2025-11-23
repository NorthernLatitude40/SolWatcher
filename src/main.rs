use anyhow::Result;
use axum::{
    routing::{get, post},
    Json, Router,
    extract::Query,
    http::StatusCode,
    Server,
    response::Json as JsonResponse,
};
use std::net::SocketAddr;
use serde_json::Value;
use tower_http::cors::{Any, CorsLayer};
use http::HeaderValue;
mod parsers;
use parsers::swap_parser::parse_swap_log;
mod models;
mod services;
use services::{
    transaction_service::fetch_transaction,
    pool_service::fetch_pools_info,
    serum_service::fetch_open_orders,
    plmint_accounts_service::fetch_plmint_accounts,
};
use listener::websocket::{
    start_subscriber,
    Configuration,
};
use dotenv::dotenv;
mod listener; // 引入 listener 模块
use crate::listener::utils::Logger;

use serde::Deserialize;

#[derive(Deserialize)]
struct SerumPoolQuery {
    addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())        // 允许所有源访问
    .allow_methods(Any)          // 允许所有方法 GET/POST/...
    .allow_headers(Any);         // 允许所有请求头
    // 定义路由
    let app = Router::new()
        .route("/api/swap_log", get(get_swap_log))
        .route("/api/transaction", get(get_transaction))
        .route("/api/pools_info", get(get_pools_info))
        .route("/api/pool_ids", post(get_pool_ids))
        .route("/api/solbalance_id", get(get_solbalance_id))
        .route("/api/open_orders", get(get_open_orders))
        .route("/api/serum_info", get(get_serum_info))
        .route("/api/serum_pools", get(get_serum_pools))
        .route("/api/plmint_accounts", get(get_plmint_accounts))
        .layer(cors);     

  // 启动 WebSocket
  tokio::spawn(async {
    if let Err(e) = start_websocket().await {
        eprintln!("WebSocket error: {:?}", e);
    }
});

    // 启动 HTTP 服务
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server running on http://{}/", addr);
    Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .map_err(|err| anyhow::anyhow!("Server error: {}", err))?;

    Ok(())
}

// 这三个 API 路由函数
pub async fn get_swap_log() -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match parsers::swap_parser::parse_swap_log().await  {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_transaction() -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::transaction_service::fetch_transaction().await {
        Ok(tx) => Ok(Json(tx.into())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_pools_info() -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::pool_service::fetch_pools_info().await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Deserialize)]
struct PoolRequest {
    ids: Vec<String>,
}
async fn get_pool_ids(Json(payload): Json<PoolRequest>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::pool_service::fetch_pool_ids(payload.ids).await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Deserialize)]
struct PoolQuery {
    id: String, 
}
async fn get_solbalance_id(Query(params): Query<PoolQuery>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::pool_service::fetch_solbalance_id(params.id).await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}



async fn get_open_orders() -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::serum_service::fetch_open_orders().await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_serum_info() -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::serum_service::fetch_serum_info().await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_serum_pools(Query(params): Query<SerumPoolQuery>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let addr = &params.addr;
    match services::serum_service::fetch_serum_pool_list(addr).await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_plmint_accounts(Query(params): Query<SerumPoolQuery>) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let addr = &params.addr;
    match services::plmint_accounts_service::fetch_plmint_accounts(addr).await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn start_websocket() -> Result<()> {
    dotenv().ok();
    let config = Configuration::new(); 
    let logger = Logger::new("Setup".to_string());

    logger.log(format!("Solana RPC websocket: {:?}", config.wss_url.as_str()));
    logger.log(format!("Solana RPC http: {:?}", config.https_url.as_str()));
    logger.log(format!("Log instruction: {:?}", config.log_instruction.as_str()));

    start_subscriber().await
}