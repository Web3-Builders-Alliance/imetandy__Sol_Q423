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
    //
    // Initialize
    //
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, fee, authority)
    }

    //
    // Deposit
    //
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64
    
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn deposit_tokens(
        ctx: Context<Deposit>,
        is_x: bool,
        amount: u64
    ) -> Result<()> {
        ctx.accounts.deposit_tokens(is_x, amount)

    }

    pub fn mint_lp_tokens(
        ctx: Context<Deposit>,
        amount: u64
    ) -> Result<()> {
        ctx.accounts.mint_lp_tokens(amount)

    }

    //
    // Withdraw
    //

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        min_x: u64,
        min_y: u64,
        expiration: i64,
        ) -> Result<()> {
            ctx.accounts.withdraw(amount, min_x, min_y, expiration)
        }

        pub fn withdraw_tokens(
            ctx: Context<Withdraw>,
            is_x: bool,
            amount: u64,
        ) -> Result<()> {
            ctx.accounts.withdraw_tokens(is_x, amount)
        }
    
        pub fn burn_lp_tokens(
            ctx: Context<Withdraw>,
            amount: u64,
    
        ) -> Result<()> {
            ctx.accounts.burn_lp_tokens(amount)
        }





        
}