use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use byteorder::{LittleEndian, ReadBytesExt};
use serde_json::json;
use std::io::Cursor;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fmt;

mod parsers;
use parsers::swap_parser::{parse_swap_log};
mod models;
use models::Pool::Pool;

mod services;
use services::{
    transaction_service::{fetch_transaction},
    pool_service::fetch_pools_info,
};


//账户关联：
// Instruction 数据只存参数
// 实际操作的 token / pool / user 都在 transaction 的 accountKeys 中
fn main() -> anyhow::Result<()> {



    let result = parse_swap_log()?;
 
    // ===========================================================

    fetch_transaction()?;


 // ===========================================================

 fetch_pools_info()?;

    Ok(())
}