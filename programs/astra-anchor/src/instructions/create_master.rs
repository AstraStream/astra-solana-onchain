use anchor_lang::prelude::*;

use crate::state::Master;

#[derive(Accounts)]
pub struct CreateMaster<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        seeds=[b"master"],
        space=8 + Master::INIT_SPACE,
        bump
    )]
    pub master_account: Account<'info, Master>,
    pub system_program: Program<'info, System>
}

pub fn handler(ctx: Context<CreateMaster>) -> Result<()> {
    msg!("initialize master account");
    ctx.accounts.master_account.set_inner(Master {
        authority: *ctx.accounts.signer.key,
        total_pools: 0,
        bump: ctx.bumps.master_account
    });
    Ok(())
}
