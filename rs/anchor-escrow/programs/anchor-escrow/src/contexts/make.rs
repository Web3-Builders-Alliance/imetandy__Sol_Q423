use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};
use crate::state::Escrow;
    
//////////////////
///   DEPOSIT  ///
//////////////////
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    // Signer
    #[account(mut)]
    maker: Signer<'info>,
    // Mints
    mint_a: Account<'info, Mint>, 
    mint_b: Account<'info, Mint>,
    // ata
    #[account(
        mut, 
        associated_token::mint = mint_a, 
        associated_token::authority = maker)]
    maker_ata_a: Account<'info, TokenAccount>,
    // Vault
    #[account(
        init, 
        payer = maker, 
        associated_token::mint = mint_a, 
        associated_token::authority = escrow)]
    vault: Account<'info,TokenAccount>,
    // Escrow
    #[account(
        init, 
        payer  = maker, 
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()], 
        bump, 
        space = Escrow::INIT_SPACE
    )]
    escrow: Account<'info, Escrow>,
    // Program accounts
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl<'info> Make<'info>  {
    pub fn deposit(&mut self , amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            transfer_accounts
        );
        transfer(ctx, amount)
    }

    pub fn save_escrow(&mut self, amount: u64, seed: u64, bump: u8) -> Result<()> {
        self.escrow.set_inner(
            Escrow {
                amount,
                seed,
                escrow_bump: bump,
                mint_a: self.mint_a.key(),
                mint_b: self.mint_b.key(),
            }
        );

        Ok(())
    }
}
