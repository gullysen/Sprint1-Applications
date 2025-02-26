use anchor_lang::prelude::*;

use super::IcoState;

#[account]
#[derive(Default)]
pub struct UserPurchase {
    pub index: u64,
    pub user_addr: Pubkey,
    pub ico: Pubkey,
    pub amount: u64,
    pub start: i64,
    pub total_claimed: u64,
}

impl UserPurchase {
    pub const DATA_SIZE: usize = 8 + std::mem::size_of::<UserPurchase>();
}