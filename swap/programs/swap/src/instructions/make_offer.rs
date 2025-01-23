use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use super::transfer_tokens;
use crate::{Offer, ANCHOR_DISCRIMINATOR};

/// Sends the offered tokens to the vault account.
pub fn send_offered_tokens_to_vault(
    context: &Context<MakeOffer>,
    token_a_offered_amount: u64,
) -> Result<()> {
    transfer_tokens(
        &context.accounts.maker_token_account_a,
        &context.accounts.vault,
        &token_a_offered_amount,
        &context.accounts.token_mint_a,
        &context.accounts.maker,
        &context.accounts.token_program,
    )
}

/// Saves the offer to the account on chain.
pub fn save_offer(context: Context<MakeOffer>, id: u64, token_b_wanted_amount: u64) -> Result<()> {
    context.accounts.offer.set_inner(Offer {
        id,
        maker: context.accounts.maker.key(),
        token_mint_a: context.accounts.token_mint_a.key(),
        token_mint_b: context.accounts.token_mint_b.key(),
        token_b_wanted_amount,
        bump: context.bumps.offer,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    // The maker of the offer 
    #[account(mut)]
    pub maker: Signer<'info>,

    // The mint of token A
    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    // The mint of token B
    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    // The maker's token account for token A
    #[account(mut, associated_token::mint = token_mint_a, associated_token::authority = maker, associated_token::token_program = token_program)]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    // The offer account
    #[account(init, payer = maker, space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE, seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()], bump)]
    pub offer: Account<'info, Offer>,

    // The vault account to hold the offered tokens
    #[account(init, payer = maker, associated_token::mint = token_mint_a, associated_token::authority = offer, associated_token::token_program = token_program)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // Used to create accounts
    pub system_program: Program<'info, System>,

    // Used for mint accounts
    pub token_program: Interface<'info, TokenInterface>,

    // Maps associated token accounts to their mint
    pub associated_token_program: Program<'info, AssociatedToken>,
}
