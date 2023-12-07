use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};
    
    pub fn deposit(ctx: Context<Deposit>, seed: u64, deposit_amount: u64, offer_amount: u64) -> Result<()> {
        Ok(())
    }


//////////////////
///   DEPOSIT  ///
//////////////////
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    maker_token_mint: Account<'info, Mint>, 
    taker_token_mint: Account<'info, Mint>,
    #[account(mut, associated_token::mint = maker_token_mint, associated_token::authority = maker)]
    maker_ata_account: Account<'info, TokenAccount>,
    #[account(init, payer = maker, associated_token::mint = maker_token_mint, associated_token::authority = auth)]
    vault: Account<'info,TokenAccount>,
    #[account(init, payer  = maker, seeds = [b"escrow", seed.to_le_bytes().as_ref()], bump, space = Escrow::LEN)]
    escrow: Account<'info, Escrow>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    #[account(seeds = [b"auth"], bump)]
    auth: UncheckedAccount<'info>,
}


impl Escrow {
    const LEN: usize = 8 + 32*3 + 8*2 + 1*3;
}
#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub maker_token_mint: Pubkey,
    pub taker_token_mint: Pubkey,
    pub amount: u64,
    pub seed: u64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub escrow_bump: u8
}
