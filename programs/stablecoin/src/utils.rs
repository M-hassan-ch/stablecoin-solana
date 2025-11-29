use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token_2022::{mint_to, MintTo, Token2022}, token_interface::{Mint, TokenAccount}};

use crate::SEEDS_MINT_ACCOUNT;

pub fn mint_tokens<'info>(
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    amount: u64,
    mint_account_bump: u8
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEEDS_MINT_ACCOUNT, &[mint_account_bump]]];

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(), 
        MintTo{
            authority: mint_account.to_account_info(),
            mint: mint_account.to_account_info(),
            to: token_account.to_account_info()
        },
        signer_seeds
    );
    mint_to(cpi_ctx, amount)
}

pub fn deposit_sol<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    amount: u64
) -> Result<()>{
    let cpi_ctx = CpiContext::new(system_program.to_account_info(), Transfer{
        from: from.to_account_info(),
        to: to.to_account_info()
    });
    transfer(cpi_ctx, amount)
}