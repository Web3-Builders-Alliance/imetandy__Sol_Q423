use anchor_lang::prelude::*;

declare_id!("96WqQbjpABLV6YRX9KVAvcYozYnT9Da7QkpujjqGftCZ");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.escrow.set_inner(Escrow {
            seed,
            mint_a: ctx.accounts.mint_a.key(),
            mint_b: ctx.accounts.mint_b.key(),
            receive,
            bump: ctx.bumps.escrow,
            vault_bump: ctx.bumps.vault,
        });

        let transfer_accounts = Transfer {
            from: ctx.accounts.maker_ata_a.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            transfer_accounts
        );

        transfer(cpi_context, deposit);

    }



    
    pub fn take(ctx: Context<Initialize>) -> Result<()> {
        Ok(())



    }
    pub fn refund(ctx: Context<Refund> ) -> Result<()> {
        let transfer_accounts = Transfer {
            from: ctx.accounts.maker_ata_a.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.maker.to_account_info(),
        };
        let binding = ctx.accounts.escrow.seed.to_le_bytes();

        let signer_seeds = &[
            b"escrow",
            ctx.accounts.maker.key().as_ref(),
            &ctx.accounts.escrow.seed.to_le_bytes()[..],
        ];

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            transfer_accounts


        );

        transfer(cpi_context, ctx.accounts.vault.amount);

    }
}



#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>
    mint_a: Account<'info, Mint>,
    mint_b: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    maker_ata_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE,
        seeds=[b"escrow", maker.key().asref(), see_to_le_bytes().as_ref()],
        bump

    )]
    authority: UncheckedAccount<'info>,
    escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    vault: Account<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>
    token_program: Program<'info, Token>
    system_program: Program<'info, System>
}





pub struct Escrow {
    seed: u64,
    mint_a: Pubkey,
    mint_b: Pubkey,
    receive: u64,
    bump: u8,
    vault_bump: u8


}
pub struct Initialize {}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 8 + 1 + 1;
}

impl<'info>