use crate::*;

#[derive(Accounts)]
pub struct RescueToken<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [ICO_POT_SEED.as_ref()],
        bump
    )]
    pub ico_state: Account<'info, IcoState>,

    #[account(init,
        mut,
        seeds = [USER_PURCHASE_SEED.as_ref()],
        bump
    )]
    pub user_purchases: Account<'info, UserPurchase>,

}

impl RescueToken<'_> {
    pub fn process_instruction(ctx: &mut Context<Self>, seed: u64) -> Result<()> {
        let ico_state = &mut ctx.accounts.ico_state;
        let user_purchases = &mut ctx.accounts.user_purchases;


        Ok(())
    }
}
