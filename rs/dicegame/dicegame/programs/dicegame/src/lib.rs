use anchor_lang::prelude::*;

declare_id!("FgVujCksmyUpP8bDxv3m4KXoHZ5HC2Jdq4n9PGzf2TYn");

#[program]
pub mod dicegame {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
