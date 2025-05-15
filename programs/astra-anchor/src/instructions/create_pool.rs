use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    error::ErrorCode,
    state::{Master, Pool},
};

#[derive(Accounts)]
pub struct CreatePool<'info> {
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
        init,
        payer=signer,
        seeds=[b"pool", mint.key().as_ref()],
        space=8 + Pool::INIT_SPACE,
        bump
    )]
    pub pool_account: Account<'info, Pool>,

    #[account(
        init,
        payer=signer,
        token::mint=mint,
        token::authority=pool_token_account,
        seeds=[b"pool_vault", mint.key().as_ref()],
        bump
    )]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreatePool>) -> Result<()> {
    msg!("create token pool for mint {}", ctx.accounts.mint.key());
    ctx.accounts.pool_account.set_inner(Pool {
        total_claimed: 0,
        balance: 0,
        mint: ctx.accounts.mint.key(),
        pool_token_account: ctx.accounts.pool_token_account.key(),
        pool_token_account_bump: ctx.bumps.pool_token_account,
        bump: ctx.bumps.pool_account,
    });

    // increment token pools in contract
    let master_account: &mut Account<Master> = &mut ctx.accounts.master_account;
    master_account.total_pools += 1;

    Ok(())
}
