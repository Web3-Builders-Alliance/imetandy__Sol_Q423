use anchor_lang::prelude::*;
use crate::{state::Config, errors::AmmError};    

#[derive(Accounts)]
        pub struct Update<'info> {
    #[account(mut)]
        pub user: Signer<'info>,
    #[account(mut, seeds = [b"config", config.seed.to_le_bytes().as_ref()],
    bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
}

impl<'info> Update<'info> {
    pub fn lock (&mut self) -> Result<()> {
        require!(self.config.authority.unwrap() == self.user.key(), AmmError::InvalidAuthority);
    
        match self.config.locked {
            true => msg!("Already Locked!"),
            false => self.config.locked = true,
        }
        Ok(())
    }
    pub fn unlock (&mut self) -> Result<()> {
        require!(self.config.authority.unwrap() == self.user.key(), AmmError::InvalidAuthority);
    
        match self.config.locked {
            true => self.config.locked = false,
            false => msg!("Already Unlocked!"),
        }
        Ok(())
    }
}