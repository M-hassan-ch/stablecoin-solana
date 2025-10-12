use anchor_lang::prelude::*;
use crate::{Config, SEEDS_CONFIG_ACCOUNT};

#[derive(Accounts)]
pub struct UpdateConfig <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEEDS_CONFIG_ACCOUNT],
        bump = config_account.bump
    )]
    pub config_account: Account<'info, Config>
}

pub fn handler(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
    msg!("Called update config function {:?}", ctx.program_id);

    let config = &mut ctx.accounts.config_account;

    config.min_health_factor = min_health_factor;

    msg!("Config account updated {:?}", config.key());
    Ok(())
}
