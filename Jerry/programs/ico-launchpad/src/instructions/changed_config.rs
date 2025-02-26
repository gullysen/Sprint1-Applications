use crate::*;

#[derive(Accounts)]
pub struct ChangeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_pool: Account<'info, GlobalPool>,
}

impl ChangeConfig<'_> {
    pub fn process_instruction(ctx: &mut Context<Self>, paused: u8) -> Result<()> {
        let global_pool = &mut ctx.accounts.global_pool;

        // Validate super admin
        require!(global_pool.admin.eq(&ctx.accounts.admin.key), LaunchpadError::InvalidAdmin);

        global_pool.is_paused = paused;

        Ok(())
    }
}