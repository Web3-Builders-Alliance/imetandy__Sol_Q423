use anchor_lang::prelude::*;

use anchor_spl::{token_interface::{Mint, TokenInterface, TokenAccount}, associated_token::AssociatedToken, token::{TransferChecked, Burn, burn, transfer_checked}};
use constant_product_curve::ConstantProduct;
use crate::{state::Config, assert_not_expired, assert_non_zero};
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"config", 
            config.key().as_ref()
        ],
        bump = config.lp_bump
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_vault_lp: InterfaceAccount<'info, TokenAccount>,
    ///CHECK: pda used just per signing purposes
    #[account(seeds =
        [b"auth"],
        bump = config.auth_bump)
    ]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x, // important
        has_one = mint_y, // important
        seeds = [
            b"config", 
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


impl<'info> Withdraw <'info> {
    pub fn withdraw(
    &mut self,
    amount: u64,
    min_x: u64,
    min_y: u64,
    expiration: i64,
    ) -> Result<()> {
        assert_not_expired!(expiration);
        assert_non_zero!([amount, min_x, min_y]);
    
        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            amount,
            0,
        ).map_err(AmmError::from)?;
  
        require!(amounts.x <= min_x && amounts.y <= min_y, AmmError::SlippageExceeded);
        self.withdraw_tokens(true, amounts.x);
        self.withdraw_tokens(false, amounts.y);
        self.burn_lp_tokens(amount);

    Ok(())
    }

    pub fn withdraw_tokens(
        &mut self,
        is_x: bool,
        amount: u64,
    ) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(), 
                self.user_vault_x.to_account_info(), 
                self.mint_x.to_account_info(),
                self.mint_x.decimals
            ),
            false => (
                self.vault_y.to_account_info(), 
                self.user_vault_y.to_account_info(), 
                self.mint_y.to_account_info(), 
                self.mint_x.decimals
            ),
        };

        let seeds = &[
            &b"auth"[..],
            &[self.config.auth_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            cpi_accounts,
            signer_seeds
        );
        transfer_checked(ctx, amount, decimals);
        
        Ok(())
    }


    pub fn burn_lp_tokens(
        &self,
        amount: u64,

    ) -> Result<()> {
        let accounts = Burn {
            from: self.user_vault_lp.to_account_info(),
            mint: self.mint_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };
        
        let ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            accounts
        );

        // burning instead of minting
        burn(ctx, amount)
        
    }

}

