use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::state::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use super::transfer_tokens;

/// Sends the offered tokens to the vault account.
pub fn send_offered_tokens_to_vault(
    context: &Context<MakeOffer>,
    token_a_offered_amount: u64,
) -> Result<()> {
    transfer_tokens(
        &context.accounts.token_maker_account_a,
        &context.accounts.vault,
        &token_a_offered_amount,
        &context.accounts.token_mint_a,
        &context.accounts.maker,
        &context.accounts.token_program,
    )
}

pub fn save_offer() -> Result<()> {
    // This is a stub function that will be filled in later
    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    // The maker of the offer
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(mut, associated_token::mint = token_mint_a, associated_token::authority = maker, associated_token::token_program = token_program)]
    pub token_maker_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(init, payer = maker, space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE, seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()], bump)]
    pub offer: Account<'info, Offer>,

    #[account(init, payer = maker, associated_token::mint = token_mint_a, associated_token::authority = offer, associated_token::token_program = token_program)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Used to create accounts
    pub system_program: Program<'info, System>,
    // Used for mint accounts
    pub token_program: Interface<'info, TokenInterface>,
    // Maps associated token accounts to their mint
    pub associated_token_program: Program<'info, AssociatedToken>,
}
