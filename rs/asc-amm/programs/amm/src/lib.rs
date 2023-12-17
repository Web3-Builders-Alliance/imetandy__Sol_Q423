use anchor_lang::prelude::*;

declare_id!("5XczppKs1Uc4z6eiy2abvhk1985Q38uxKSqKwCCDYsL2");

pub mod state;
pub mod contexts;
pub mod errors;
pub mod helpers;

pub use state::*;
pub use contexts::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority)
    }
}
