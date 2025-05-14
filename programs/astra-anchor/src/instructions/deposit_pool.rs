use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{
    state::{Master, Pool},
    error::ErrorCode
};

#[derive(Accounts)]
pub struct DepositPool<'info> {
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
        constraint=pool_account.mint == mint.key() @ ErrorCode::InvalidMint,
        seeds=[b"pool", mint.key().as_ref()],
        bump=pool_account.bump
    )]
    pub pool_account: Account<'info, Pool>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<DepositPool>, amount: u64) -> Result<()> {
    let pool_account: &mut Account<Pool> = &mut ctx.accounts.pool_account;
    
    require!(ctx.accounts.vault_ata.amount >= amount, ErrorCode::InsufficientFunds);

    // transfer token to pool
    token_interface::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_ata.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.signer.to_account_info()
            },
        ),
        amount,
        ctx.accounts.mint.decimals
    )?;

    // update pool state
    pool_account.balance += amount;
    Ok(())    
}