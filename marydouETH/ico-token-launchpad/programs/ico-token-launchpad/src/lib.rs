use anchor_lang::prelude::*;

declare_id!("6P52eNzyNVg7aU5PDJaVm1jrcHB3PEC15G8FcZB9QiTz");

#[program]
pub mod ico_token_launchpad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
