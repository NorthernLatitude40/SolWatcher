use anyhow::Result;
use axum::{
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use serde_json::Value;
use axum::http::StatusCode;
use axum::Server;
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
};

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
        .route("/api/open_orders", get(get_open_orders))
        .layer(cors);     

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

async fn get_open_orders() -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    match services::serum_service::fetch_open_orders().await {
        Ok(val) => Ok(Json(val.into())),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}