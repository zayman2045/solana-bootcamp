use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use super::transfer_tokens;
use crate::{Offer, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
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
    #[account(mut, close = maker)]
    pub offer: Account<'info, Offer>,

    // Used to create accounts
    pub system_program: Program<'info, System>,

    // Used for mint accounts
    pub token_program: Interface<'info, TokenInterface>,

    // Maps associated token accounts to their mint
    pub associated_token_program: Program<'info, AssociatedToken>,
}
