#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod crud {
    use super::*;

    pub fn create_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.owner = ctx.accounts.owner.key();
        entry.title = title;
        entry.message = message;
        Ok(())
    }

    pub fn update_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.message = message;
        Ok(())
    }

    pub fn delete_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner,space = 8 + Entry::INIT_SPACE, seeds = [title.as_bytes(), owner.key().as_ref()], bump)]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {
    #[account(mut, realloc = 8 + Entry::INIT_SPACE, realloc::payer = owner, realloc::zero = true,  seeds = [title.as_bytes(), owner.key().as_ref()], bump)]
    pub entry: Account<'info, Entry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    #[account(mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()], bump,
        close = owner)]
    pub entry: Account<'info, Entry>,
    #[account(mut)]
    pub owner: Signer<'info>,
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
