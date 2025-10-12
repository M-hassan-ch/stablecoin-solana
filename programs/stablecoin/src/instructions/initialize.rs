use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use crate::{Config, LIQUIDATION_BONUS, LIQUIDATION_THRESHOLD, MINT_DECIMALS, MIN_HEALTH_FACTOR, SEEDS_CONFIG_ACCOUNT, SEEDS_MINT_ACCOUNT};

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [SEEDS_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEEDS_CONFIG_ACCOUNT],
        bump
    )]
    pub config_account: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: This is the token program and is validated by the mint account constraint
    pub token_program: AccountInfo<'info >,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Called initialize function {:?}", ctx.program_id);

    let config = &mut ctx.accounts.config_account;

    config.set_inner(Config {
        authority: ctx.accounts.signer.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account,
    });

    msg!("Config account initialized {:?}", config.key());
    Ok(())
}
