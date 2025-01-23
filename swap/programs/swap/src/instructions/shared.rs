pub use anchor_lang::prelude::*;


pub fn transfer_tokens<'info>(
    // A way to have our program work across tokens that are made with both the older and newer token programs
    from: InterfaceAccount<'info, TokenAccount>,
) -> Result<()> {

}