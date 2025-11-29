use anchor_lang::prelude::*;
use anchor_spl::{associated_token::{AssociatedToken}, token_2022::Token2022, token_interface::{Mint, TokenAccount}};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{deposit_sol, mint_tokens, Collateral, Config, SEEDS_CONFIG_ACCOUNT, SEEDS_SOL_ACCOUNT};

#[derive(Accounts)]
pub struct DepositCollateral<'info>{
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEEDS_CONFIG_ACCOUNT],
        bump,
        has_one = mint_account
    )]
    pub config_account: Account<'info, Config>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = depositer,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEEDS_CONFIG_ACCOUNT, depositer.key().as_ref()],
        bump
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [SEEDS_SOL_ACCOUNT, depositer.key().as_ref()],
        bump
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositer,
        associated_token::mint = mint_account,
        associated_token::authority = depositer,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<DepositCollateral>,
    amount_collateral: u64,
    amount_to_mint: u64
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;

    if !collateral_account.is_initialized {
        collateral_account.depositer = ctx.accounts.depositer.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.bump_sol_account = ctx.bumps.sol_account;
        collateral_account.is_initialized = true;
    }
    
    deposit_sol(&ctx.accounts.depositer, &ctx.accounts.sol_account, &ctx.accounts.system_program, amount_collateral)?;

    mint_tokens(&ctx.accounts.mint_account, &ctx.accounts.token_account, &ctx.accounts.token_program, amount_to_mint, ctx.accounts.config_account.bump_mint_account)?;

    Ok(())
}