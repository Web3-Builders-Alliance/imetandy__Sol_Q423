use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer, CloseAccount, close_account}, associated_token::AssociatedToken};
use crate::state::Escrow;

//////////////////
///   Take     ///
//////////////////
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    // Taker
    #[account(mut)]
    taker: Signer<'info>,
    // Maker
    #[account(mut)]
    maker: SystemAccount<'info>,
    // Mints
    mint_a: Account<'info, Mint>, 
    mint_b: Account<'info, Mint>,
    // ATAs 
    // Create ATA for take for mint_b
    // Create ATA for taker for mint_a if req
    // Create ATA for maker for mint_b if req
    #[account(
        mut, 
        associated_token::mint = mint_b, 
        associated_token::authority = taker
    )]
    taker_ata_b: Account<'info, TokenAccount>,

    #[account(
        init_if_needed, 
        payer = taker,
        associated_token::mint = mint_a, 
        associated_token::authority = maker
    )]
    taker_ata_a: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker
    )]
    maker_ata_b: Account<'info, TokenAccount>,
    // Escrow
    #[account(
        mut,
        close = maker,
        seeds=[b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump,
        has_one = mint_a,
        has_one = mint_b
    )]
    escrow: Account<'info, Escrow>,
    // Vault
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker
    )]
    vault: Account<'info,TokenAccount>,
    // Program Accounts
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl<'info> Take<'info>  {

    pub fn deposit(&mut self) -> Result<()> {
        let tranfer_accounts = Transfer {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            tranfer_accounts
        );
        transfer(ctx, self.escrow.amount)
    }

    pub fn withdraw(&mut self) -> Result<()> {
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
            to: self.taker_ata_a.to_account_info(),
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
            destination: self.taker.to_account_info(),
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