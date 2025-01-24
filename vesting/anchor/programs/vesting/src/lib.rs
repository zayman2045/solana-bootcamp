#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod vesting {
    use super::*;

    pub fn create_vesting_account(ctx: Context<CreateVestingAccount>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = ANCHOR_DISCRIMINATOR + VestingAccount::INIT_SPACE)]
    pub vesting_account: Account<'info, VestingAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}
