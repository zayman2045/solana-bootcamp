#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub const ANCHOR_DISCRIMINATOR: usize = 8;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod vesting {
    use super::*;

    /// Initializes a new vesting account and treasury token account, and updates the vesting account data
    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
          owner: ctx.accounts.signer.key(),
          mint: ctx.accounts.mint.key(),
          treasury_token_account: ctx.accounts.treasury_token_account.key(),
          company_name,
          treasury_bump: ctx.bumps.treasury_token_account,
          bump: ctx.bumps.vesting_account,
        };
        Ok(())
    }
}

/// Initializes a vesting account and a token account to act as the treasury
#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // This account is used to store the vesting account data
    #[account(init, payer = signer, space = ANCHOR_DISCRIMINATOR + VestingAccount::INIT_SPACE, seeds = [company_name.as_ref()], bump)]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    // This is a token account, not an associated token account. This is because it is specified just for this vesting contract.
    #[account(
      init, 
      token::mint = mint, 
      token::authority = treasury_token_account, 
      payer = signer, 
      seeds = [b"vesting_treasury", 
      company_name.as_bytes()], 
      bump 
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

/// Contains the vesting account data
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
