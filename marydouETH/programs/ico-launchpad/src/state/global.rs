use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
}

impl GlobalPool {
    pub const DATA_SIZE: usize = 8 + std::mem::size_of::<GlobalPool>();
}
