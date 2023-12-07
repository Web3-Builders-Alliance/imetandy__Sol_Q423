use anchor_lang::prelude::*;

declare_id!("G97vWGkrWRCDDnP1MaNJDLF5cewWjXv6NP23hoADRPvf");

#[program]
pub mod anchor_lang_docs {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.my_account.data = 0;
        ctx.accounts.my_account.bump = ctx.bumps.my_account;
        Ok(())
    }
 


    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        ctx.accounts.my_account.data = data;
        Ok(())
    }
}


#[account]
#[derive(Default)]
pub struct MyAccount {
    pub data: u64,
    pub bump: u8
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = payer, 
        seeds = [
            b"my_account", 
            payer.key().as_ref()
            ],
        bump,
        space = 8 + 8 + 1
    )]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(
        mut,
        seeds = [
            b"my_account", 
            payer.key().as_ref()],
            bump = my_account.bump,
    )]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}