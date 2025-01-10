#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

const ANCHOR_SPACE_DISCRIMINATOR: usize = 8;

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(_ctx: Context<InitializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space=ANCHOR_SPACE_DISCRIMINATOR + Poll::INIT_SPACE, seeds=[poll_id.to_le_bytes().as_ref()], bump)]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64
}