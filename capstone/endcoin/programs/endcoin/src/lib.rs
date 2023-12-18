use anchor_lang::prelude::*;

declare_id!("FRF9LYrnAH7MKTgXpxTkKPHikC8YFBWgNACqrTTQe1A8");

#[program]
pub mod endcoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
