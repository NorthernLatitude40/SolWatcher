use solana_sdk::pubkey::Pubkey;
use serde::Serialize;

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct AmmInfo {
    pub status: u8,
    pub nonce: u8,
    pub order_num: u64,
    pub lp_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub open_orders: Pubkey,
    // 这里可以加其他字段
}
