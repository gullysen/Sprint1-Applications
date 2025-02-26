use crate::*;

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [ICO_POT_SEED.as_ref()],
        bump
    )]
    pub ico_state: Account<'info, IcoState>,

    #[account(init,
        space = IcoState::DATA_SIZE,
        seeds = [USER_PURCHASE_SEED.as_ref()],
        bump,
        payer = user
    )]
    pub user_purchases: Account<'info, UserPurchase>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl BuyToken<'_> {
    pub fn process_instruction(ctx: &mut Context<Self>, seed: u64, amount: u64) -> Result<()> {
        let ico_state = &mut ctx.accounts.ico_state;
        let user_purchases = &mut ctx.accounts.user_purchases;


        Ok(())
    }
}
