mod contexts;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};



declare_id!("96WqQbjpABLV6YRX9KVAvcYozYnT9Da7QkpujjqGftCZ");

#[program]
pub mod anchor_escrow {
    use super::*;
    use contexts::deposit::Deposit;



}