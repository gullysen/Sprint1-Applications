use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct CreateIcoParams {
}

#[account]
#[derive(Default)]
pub struct IcoState {
}

impl IcoState {
    pub const DATA_SIZE: usize = 8 + std::mem::size_of::<IcoState>();}
