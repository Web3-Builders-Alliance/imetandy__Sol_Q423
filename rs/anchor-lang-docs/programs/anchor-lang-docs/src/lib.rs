use anchor_lang::prelude::*;

declare_id!("G97vWGkrWRCDDnP1MaNJDLF5cewWjXv6NP23hoADRPvf");

#[program]
pub mod anchor_lang_docs {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.my_account.data = 0;
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
    pub data: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = payer, 
        space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut, seeds = [
        b"my_account".as_ref(),
        my_account.key().as_ref()
    ], bump)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

