use crate::*;

#[derive(Accounts)]
pub struct CloseIco<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [ICO_POT_SEED.as_ref()],
        bump
    )]
    pub ico_state: Account<'info, IcoState>,

}

impl CloseIco<'_> {
    pub fn process_instruction(ctx: &mut Context<Self>, seed: u64) -> Result<()> {
        let ico_state = &mut ctx.accounts.ico_state;

        // Validate super admin
        require!(ico_state.owner.eq(&ctx.accounts.owner.key), LaunchpadError::InvalidIcoOwner);

        // Validate If ICO is closed or not
        require!(ico_state.closed != 0, LauchpadError::AlreadyClose);

        ico_state.closed = 0;

        Ok(())
    }
}