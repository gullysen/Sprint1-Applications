use anchor_lang::prelude::*;

use super::IcoState;

#[account]
#[derive(Default)]
pub struct UserPurchase {
}

impl UserPurchase {
    pub const DATA_SIZE: usize = 8 + std::mem::size_of::<UserPurchase>();
}
