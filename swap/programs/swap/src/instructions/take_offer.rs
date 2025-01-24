use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{close_account, CloseAccount, Mint,  TokenAccount, TokenInterface, TransferChecked, transfer_checked}
};

use super::transfer_tokens;
use crate::Offer;

/// Sends the wanted tokens to the maker's account.
pub fn send_wanted_tokens_to_maker(context: &Context<TakeOffer>) -> Result<()> {
    transfer_tokens(
        &context.accounts.taker_token_account_b,
        &context.accounts.maker_token_account_b,
        &context.accounts.offer.token_b_wanted_amount,
        &context.accounts.token_mint_b,
        &context.accounts.taker,
        &context.accounts.token_program,
    )
}

/// Withdraws the tokens from the vault and closes it.
pub fn withdraw_and_close_vault(context: Context<TakeOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        context.accounts.maker.to_account_info().key.as_ref(),
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump]
    ];

    let signer_seeds = [&seeds[..]];

    let accounts = TransferChecked {
        from: context.accounts.vault.to_account_info(),
        mint: context.accounts.token_mint_a.to_account_info(),
        to: context.accounts.taker_token_account_a.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(context.accounts.token_program.to_account_info(), accounts, &signer_seeds);

    transfer_checked(cpi_context, context.accounts.vault.amount, context.accounts.token_mint_a.decimals)?;

    let accounts = CloseAccount {
        account: context.accounts.vault.to_account_info(),
        destination: context.accounts.maker.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(context.accounts.token_program.to_account_info(), accounts, &signer_seeds);

    close_account(cpi_context)
    }

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct TakeOffer<'info> {
    // The taker of the offer
    #[account(mut)]
    pub taker: Signer<'info>,

    // The maker of the offer
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // The mint of token A
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    // The mint of token B
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    // The taker's token account for token A
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a, 
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    // The taker's token account for token B
    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    // The maker's token account for token B
    #[account(init_if_needed, payer = taker, associated_token::mint = token_mint_b, associated_token::authority = maker, associated_token::token_program = token_program)]
    pub maker_token_account_b: InterfaceAccount<'info, TokenAccount>,

    // The offer account
    #[account(
        mut, 
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        has_one = token_mint_b,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    // The vault account that holds the offered tokens
    #[account(mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Used to create accounts
    pub system_program: Program<'info, System>,

    // Used for mint accounts
    pub token_program: Interface<'info, TokenInterface>,

    // Maps associated token accounts to their mint
    pub associated_token_program: Program<'info, AssociatedToken>,
}
