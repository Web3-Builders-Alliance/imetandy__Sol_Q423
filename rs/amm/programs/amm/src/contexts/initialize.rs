use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenInterface};
use crate::state::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)] pub initializer: Signer<'info>,
    
    pub mint_x: InterfaceAccount<'info, Mint>, // Allows for Token 2022!!!
    pub mint_y: InterfaceAccount<'info, Mint>, // Allows for Token 2022!!!
    
    #[account(init,
    seeds = [b"lp", config.key().as_ref()
    ],
    payer = initializer,
    bump,
    mint::decimals = 6,
    mint::authority = auth,)
    ] pub lp_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = initializer,
    )] pub vault_x: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = initializer,
    )] pub vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
    seeds = [b"auth"], bump)
    ] pub auth: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE,
    )] pub config: Account<'info, Config>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &InitializeBumps,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        
        require!(fee <= 10000, AmmError::InvalidFee);
        self.config.set_inner(
            Config {
                seed,
                authority,
                mint_x: self.mint_x.key(),
                mint_y: self.mint_y.key(),
                fee,
                locked: false,
                auth_bump: bumps.auth,
                config_bump: bumps.config,
                lp_bump: bumps.lp_mint,
            });

        Ok(())
    }
    

}