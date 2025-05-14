use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Master {
    pub authority: Pubkey,
    pub total_pools: u64,    
    pub bump: u8
}