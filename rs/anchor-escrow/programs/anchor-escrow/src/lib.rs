pub mod contexts;
pub mod state;
use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, TokenAccount, Token};
use contexts::*;


declare_id!("96WqQbjpABLV6YRX9KVAvcYozYnT9Da7QkpujjqGftCZ");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit_amount: u64, offer_amount: u64) -> Result<()> {
        ctx.accounts.deposit(deposit_amount)?;
        ctx.accounts.save_escrow(offer_amount, seed, ctx.bumps.escrow)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close_vault()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_vault()
    }

}