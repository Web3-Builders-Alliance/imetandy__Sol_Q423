use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{Mint, TokenAccount, TokenInterface}, associated_token::AssociatedToken, token::{TransferChecked, transfer_checked}};
use constant_product_curve::{ConstantProduct, LiquidityPair};
use crate::{state::Config, assert_not_expired, assert_non_zero};
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )] pub vault_x: InterfaceAccount<'info, TokenAccount>,

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


impl<'info> Swap <'info> {
    pub fn swap(
        &mut self,
        is_x: bool,
        amount: u64,
        min: u64,
        expiration: i64
    ) -> Result <()> {

        assert_not_expired!(expiration);
        assert_non_zero!([amount]);    
    
    let mut curve = ConstantProduct::init(
        self.vault_x.amount,
        self.vault_y.amount,
        self.vault_x.amount,
        self.config.fee,
        None
    );
    let pair = match is_x {
        true => LiquidityPair::X,
        false => LiquidityPair::Y,
    };

    let res = curve.swap(pair, amount, min)
    .map_err(AmmError::from)?;

    self.deposit_tokens(is_x, res.deposit)?;
    self.withdraw_tokens(is_x, res.withdraw)?;
    Ok(())

    }

    pub fn deposit_tokens(
        &mut self,
        is_x: bool,
        amount: u64,
    ) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_vault_x.to_account_info(), 
                self.vault_x.to_account_info(), 
                self.mint_x.to_account_info(),
                self.mint_x.decimals
            ),
            false => (
                self.user_vault_y.to_account_info(), 
                self.vault_y.to_account_info(), 
                self.mint_y.to_account_info(), 
                self.mint_x.decimals
            ),
        };

        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint
        };
        

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer_checked(ctx, amount, decimals);
        
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



}