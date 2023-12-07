use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount: u64,
    pub seed: u64,
    pub escrow_bump: u8
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 32*2 + 8*2 + 1;
}