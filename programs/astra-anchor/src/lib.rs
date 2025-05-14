pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5CLJvW9rbWitTVZ7h1U1XotCVZV9J6QAk8bUKFJoweUW");

#[program]
pub mod astra_play {
    use super::*;

    pub fn create_master(ctx: Context<CreateMaster>) -> Result<()> {
        create_master::handler(ctx)
    }

    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        create_pool::handler(ctx)
    }

    pub fn deposit_pool(ctx: Context<DepositPool>, amount: u64) -> Result<()> {
        deposit_pool::handler(ctx, amount)
    }

    pub fn withdraw_pool(ctx: Context<WithdrawPool>, amount: u64) -> Result<()> {
        withdraw_pool::handler(ctx, amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, username: String, role: String, amount: u64) -> Result<()> {
        claim_reward::handler(ctx, username, role, amount)
    }
}
