use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>, // wrapper around value that may or may not exist
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub auth_bump: u8,
    pub config_bump: u8,
    pub lp_bump: u8,


}

impl Space for Config {
    const INIT_SPACE: u64 = 8 + 8 + 1 + 32 + 32 + 32 + 2 + 1 + 1 + 1 + 1;
}