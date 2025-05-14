use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub mint: Pubkey,
    pub balance: u64,
    pub total_claimed: u64,
    pub pool_token_account: Pubkey,
    pub pool_token_account_bump: u8,
    pub bump: u8
}