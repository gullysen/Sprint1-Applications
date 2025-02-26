use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct CreateIcoParams {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub cost_mint: Pubkey,
    pub price_start: u64,
    pub price_end: u64,
    pub start_date: i64,
    pub end_date: i64,
    pub bonus_reserve: u64,
    pub bonus_percentage: u16,
    pub bonus_activator: u16,
    pub closed: u8,
    pub total_sold: u64,
    pub total_received: u64, 
    pub vesting_params: u16,
    pub unlocked_percentage: u16,
}

#[account]
#[derive(Default)]
pub struct IcoState {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub cost_mint: Pubkey,
    pub price_start: u64,
    pub price_end: u64,
    pub start_date: i64,
    pub end_date: i64,
    pub bonus_reserve: u64,
    pub bonus_percentage: u16,
    pub bonus_activator: u16,
    pub closed: u8,
    pub total_sold: u64,
    pub total_received: u64, 
    pub vesting_params: u16,
    pub unlocked_percentage: u16,
}

impl IcoState {
    pub const DATA_SIZE: usize = 8 + std::mem::size_of::<IcoState>();}