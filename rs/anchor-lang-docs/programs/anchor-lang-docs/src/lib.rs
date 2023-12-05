use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("9vKqa7W8GfZiCsE6SicAZuiDPbDqquU4wTsp1zfg7VYE");

#[program]
pub mod anchor_lang_docs {
    use super::*;
//                               1          5         4
    pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> Result<()> {
        require!(data.data < 100, MyError::DataTooLarge);
        //   1      2         3                 5
            ctx.accounts.my_account.set_inner(data);
            Ok(())
        }
        
    
    }


#[account]
// The data account
#[derive(Default)]
//               4
pub struct MyAccount {
//    5
    data: u64,
    mint: Pubkey,
    age: u8
}


#[derive(Accounts)]
//           1
pub struct SetData<'info> {
    // The account to update
    //   2  
    #[account(mut)]
    //      3                         4
    pub my_account: Account<'info, MyAccount>,
    #[account(
        constraint = my_account.mint == token_account.mint,
        has_one = owner,
    )]
    pub token_account: Account<'info,TokenAccount>,
    pub owner: Signer<'info>,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub potentially_dangerous: UncheckedAccount<'info>
}

#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data less than 100")]
    DataTooLarge
}