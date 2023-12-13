use anchor_lang::prelude::*;
use anchor_spl::{token_interface::TokenInterface, token::{Mint, TokenAccount}, associated_token::AssociatedToken};

use crate::state::Config;

#[derive(Accounts)]

pub struct Deposit<'info> {
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
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    ///CHECK: PDA used just for signing purposes
    pub user_vault_lp: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            [b"auth"], 
            bump: config.auth_bump
        ]
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [
            b"config",
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
