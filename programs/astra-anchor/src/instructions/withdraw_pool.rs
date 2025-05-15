use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::LAMPORTS_PER_SOL,
    error::ErrorCode,
    state::{Master, Pool},
};

#[derive(Accounts)]
pub struct WithdrawPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        constraint=master_account.authority == signer.key() @ ErrorCode::Unauthorized,
        seeds=[b"master"],
        bump=master_account.bump
    )]
    pub master_account: Account<'info, Master>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one=mint,
        has_one=pool_token_account,
        seeds=[b"pool", mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,

    #[account(mut)]
    pool_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawPool>, amount: u64) -> Result<()> {
    let pool_account: &mut Account<Pool> = &mut ctx.accounts.pool_account;

    // check if vault has sufficient funds
    require!(
        ctx.accounts.pool_token_account.amount >= amount,
        ErrorCode::InsufficientFunds
    );

    let mint_key = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"pool_vault",
        mint_key.as_ref(),
        &[pool_account.pool_token_account_bump],
    ]];

    // transfer funds from pool token vault to destination_ata
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.vault_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.pool_token_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    // update pool balance
    let amount_withdrawn: u64 = amount
        .checked_div(LAMPORTS_PER_SOL as u64)
        .ok_or(ErrorCode::MathOverflow)
        .unwrap();
    pool_account.balance -= amount_withdrawn;

    Ok(())
}
