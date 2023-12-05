use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;
declare_id!("5ZAzo76efXoLsHWEs1E6KkiRF5fTLxm6udyhEPCM682j");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<Vault>, lamports: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), accounts);

        transfer(cpi_context, lamports)
    }

    pub fn close(ctx: Context<Vault>) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        };

        let seeds = &[
            b"vault", 
            ctx.accounts.signer.to_account_info().key.as_ref(),  
            &[ctx.bumps.vault]
            ];

        let signer_seeds = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            accounts, 
            signer_seeds
        );

        transfer(cpi_context, ctx.accounts.vault.lamports())
    }
}

#[derive(Accounts)]
pub struct Vault<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(mut, seeds = [b"vault", signer.key.as_ref()], bump)]
    vault: SystemAccount<'info>,
    system_program: Program<'info, System>
}

