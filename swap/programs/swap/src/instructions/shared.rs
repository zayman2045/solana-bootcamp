use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

/// Transfers tokens from one account to another.
pub fn transfer_tokens<'info>(
    // InterfaceAccount is way to have our program work across tokens that are made with both the older and newer token programs
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        // Turning InterfaceAccount objects into AccountInfo objects
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    // The program plus the options/arguments
    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);

    // The CPI to the token program to transfer the tokens
    transfer_checked(cpi_context, *amount, mint.decimals)
}
