#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

pub const ANCHOR_DISCRIMINATOR: usize = 8;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod vesting {
    use super::*;

    /// Initializes a new vesting account and treasury token account, and sets the initial vesting account data.
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

    /// Initializes a new employee account and sets the initial employee account data.
    pub fn create_employee_account(
      ctx: Context<CreateEmployeeAccount>, 
      start_time: i64, 
      end_time: i64, 
      cliff_time: i64, 
      total_amount: u64
    ) -> Result<()> {
      *ctx.accounts.employee_account = EmployeeAccount {
        beneficiary: ctx.accounts.beneficiary.key(),
        start_time,
        end_time,
        cliff_time,
        vesting_account: ctx.accounts.vesting_account.key(),
        total_amount,
        total_withdrawn: 0,
        bump: ctx.bumps.employee_account,
      };

        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, company_name: String) -> Result<()> {
      Ok(())
    }
}

/// Initializes a vesting account and a token account to act as the treasury.
#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    // This account is used to store the vesting account data
    #[account(init, payer = signer, space = ANCHOR_DISCRIMINATOR + VestingAccount::INIT_SPACE, seeds = [company_name.as_ref()], bump)]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    // This is a token account, not an associated token account because it is specified just for this vesting contract
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

/// Initializes an employee account.
#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  pub beneficiary: SystemAccount<'info>,

  #[account(
    has_one = owner // The owner passed into the instruction must match the owner in the vesting account
  )]
  pub vesting_account: Account<'info, VestingAccount>,

  #[account(
    init, 
    payer = owner, 
    space = ANCHOR_DISCRIMINATOR + EmployeeAccount::INIT_SPACE, 
    seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()], 
    bump
  )]
  pub employee_account: Account<'info, EmployeeAccount>,

  pub system_program: Program<'info, System>
}

/// 
#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
  #[account(mut)]
  pub beneficiary: Signer<'info>,

  #[account(
    mut, 
    seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()], 
    bump = employee_account.bump,
    has_one = beneficiary, // The beneficiary passed into the instruction must match the beneficiary in the employee account
    has_one = vesting_account // The vesting account passed into the instruction must match the vesting account in the employee account
  )]
  pub employee_account: Account<'info, EmployeeAccount>,

  #[account(
    mut, 
    seeds = [company_name.as_ref()], 
    bump = vesting_account.bump,
    has_one = treasury_token_account,
    has_one = mint
  )]
  pub vesting_account: Account<'info, VestingAccount>,

  pub mint: InterfaceAccount<'info, Mint>,

  #[account(mut)]
  pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
}

/// Contains the vesting account data.
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

/// Contains the employee account data.
#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
  pub beneficiary: Pubkey,
  pub start_time: i64,
  pub end_time: i64,
  pub cliff_time: i64,
  pub vesting_account: Pubkey,
  pub total_amount: u64,
  pub total_withdrawn: u64,
  pub bump: u8,
}