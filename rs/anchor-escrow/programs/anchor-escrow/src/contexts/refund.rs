use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, CloseAccount, close_account, Transfer, transfer}, associated_token::AssociatedToken};
use crate::state::Escrow;

//////////////////
///   REFUND  ///
//////////////////
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Refund<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    // Mints
    mint_a: Account<'info, Mint>, 
    #[account(
        mut, 
        associated_token::mint = mint_a, 
        associated_token::authority = maker
    )]
    maker_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut, 
        close = maker,
        has_one = mint_a, 
        seeds = [
            b"escrow", 
            maker.key().as_ref(), 
            escrow.key().as_ref()], 
            bump = escrow.escrow_bump, 
        )]
    escrow: Account<'info, Escrow>,
    #[account(
        seeds=[
            b"vault", 
            escrow.key().as_ref()
        ],
        bump=escrow.escrow_bump,
        token::mint = mint_a, 
        token::authority = escrow)]
    vault: Account<'info,TokenAccount>,


    // Program accounts
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl<'info> Refund<'info> {

    pub fn refund(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]];1] = [
            &[
                b"escrow", 
                self.maker.to_account_info().key.as_ref(), 
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.escrow_bump]
            ]
        ];     
        let transfer_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            transfer_accounts,
            &signer_seeds
        );
        transfer(ctx, self.vault.amount)
    }
    
    pub fn close_vault(&mut self) -> Result<()> {

        let signer_seeds: [&[&[u8]];1] = [
            &[
                b"escrow", 
                self.maker.to_account_info().key.as_ref(), 
                &self.escrow.seed.to_le_bytes()[..],
                &[self.escrow.escrow_bump]
            ]
        ];

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            close_accounts,
            &signer_seeds
        );
        
        close_account(ctx)
    }
}