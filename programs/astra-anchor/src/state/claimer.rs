use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Claimer {
    pub address: Pubkey,
    #[max_len(10)]
    pub role: String,
    pub token_account: Pubkey,
    pub total_claimed: u64,
    pub last_claimed_at: i64
}