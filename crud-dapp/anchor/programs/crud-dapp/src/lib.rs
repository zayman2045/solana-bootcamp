#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod crud_dapp {
    use super::*;

    pub fn create_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.owner = ctx.accounts.user.key();
        entry.title = title;
        entry.message = message;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user,space = 8 + Entry::INIT_SPACE, seeds = [title.as_bytes(), user.key().as_ref()], bump)]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Entry {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub message: String,
}
