use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub admin: Pubkey,
    pub is_paused: u8,
}

impl GlobalPool {
    pub const DATA_SIZE: usize = 8 + std::mem::size_of::<GlobalPool>();
}