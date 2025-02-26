use crate::*;

#[derive(Accounts)]
pub struct CreateIco<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        space = IcoState::DATA_SIZE,
        seeds = [ICO_POT_SEED.as_ref()],
        bump,
        payer = owner
    )]
    pub ico_state: Account<'info, IcoState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl CreateIco<'_> {
    pub fn process_instruction(ctx: &mut Context<Self>, params: CreateIcoParams) -> Result<()> {
        let ico_state = &mut ctx.accounts.ico_state;

        ico_state.owner = ctx.accounts.owner.key();
        ico_state.token_mint = params.owner.key();
        ico_state.amount = params.amount;
        ico_state.cost_mint = params.cost_mint.key();
        ico_state.price_start = params.price_start;
        ico_state.price_end = parmas.price_end;
        ico_state.start_date = params.start_date;
        ico_state.bonus_reserve = params.bonus_reserve;
        ico_state.bonus_percentage = params.bonus_percentage;
        ico_state.bonus_activator = params.bonus_activator;
        ico_state.closed = params.closed;
        ico_state.total_sold = params.total_sold;
        ico_state.total_received = params.total_received;
        ico_state.vesting_params = params.vesting_params;
        ico_state.unlocked_percentage = params.unlocked_percentage;

        Ok(())
    }
}