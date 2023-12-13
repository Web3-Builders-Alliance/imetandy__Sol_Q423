use anchor_lang::prelude::*;
pub mod errors;

declare_id!("FVDa19xM6SJMZ7Wi8B1k2KgPSJGGA8DeDhS62aG4xmGu");

pub mod state;
pub use state::*;

pub mod contexts;
pub use contexts::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
