use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};


pub fn refund(ctx: Context<Refund> ) -> Result<()> {
    let transfer_accounts = Transfer {
        from: ctx.accounts.maker_ata_a.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };
    let binding = ctx.accounts.escrow.seed.to_le_bytes();

    let signer_seeds = &[
        b"escrow",
        ctx.accounts.maker.key().as_ref(),
        &ctx.accounts.escrow.seed.to_le_bytes()[..],
    ];

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        transfer_accounts
    );

    transfer(cpi_context, ctx.accounts.vault.amount);

}



