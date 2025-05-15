use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    constants::LAMPORTS_PER_SOL,
    error::ErrorCode,
    state::{Claimer, Pool},
};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

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

    #[account(mut)]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer=signer,
        space=8 + Claimer::INIT_SPACE,
        seeds=[b"claimer", signer.key().as_ref(), pool_account.key().as_ref()],
        bump,
    )]
    pub claimer_account: Account<'info, Claimer>,

    #[account(
        init_if_needed,
        payer=signer,
        associated_token::mint=mint,
        associated_token::authority=signer,
        associated_token::token_program=token_program
    )]
    pub claimer_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ClaimReward>,
    username: String,
    role: String,
    amount: u64,
) -> Result<()> {
    let claimer_account: &mut Account<Claimer> = &mut ctx.accounts.claimer_account;
    let pool_account: &mut Account<Pool> = &mut ctx.accounts.pool_account;

    let mint_key = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"pool_vault",
        mint_key.as_ref(),
        &[pool_account.pool_token_account_bump],
    ]];

    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.pool_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.claimer_ata.to_account_info(),
                authority: ctx.accounts.pool_token_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    let amount_earned: u64 = amount
        .checked_div(LAMPORTS_PER_SOL as u64)
        .ok_or(ErrorCode::MathOverflow)
        .unwrap();
    if claimer_account.total_claimed == 0 && claimer_account.last_claimed_at == 0 {
        // initialize claimer account
        claimer_account.address = *ctx.accounts.signer.key;
        claimer_account.username = username;
        claimer_account.role = role;
        claimer_account.token_account = ctx.accounts.claimer_ata.key();
        claimer_account.total_claimed = amount_earned;
        claimer_account.pool = pool_account.key();
        claimer_account.last_claimed_at = Clock::get()?.unix_timestamp;
    } else {
        // Update existing claimer account
        claimer_account.total_claimed += amount_earned;
        claimer_account.last_claimed_at = Clock::get()?.unix_timestamp;
    }

    // Update pool account balance
    pool_account.balance -= amount_earned;

    Ok(())
}
