use solana_sdk::pubkey::Pubkey;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Pool {
    #[serde(rename = "id")]
    pub pool_id: String,
    #[serde(rename = "poolType")]
    pub pool_type: String,
    #[serde(rename = "lpMint")]
    pub lp_mint: String,
    #[serde(rename = "baseMint")]
    pub base_mint: String,
    #[serde(rename = "quoteMint")]
    pub quote_mint: String,
    #[serde(rename = "feeTier")]
    pub fee_tier: Option<u64>,
    #[serde(rename = "totalLiquidity")]
    pub total_liquidity: Option<f64>,
    #[serde(rename = "volume")]
    pub volume: Option<f64>,
    #[serde(rename = "price")]
    pub price: Option<f64>,
}