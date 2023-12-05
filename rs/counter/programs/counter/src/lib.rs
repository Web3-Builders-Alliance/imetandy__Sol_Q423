use anchor_lang::prelude::*;

declare_id!("DPXqfPv4i5NkU4ENmgQLAfD2LWKpYLyzxFFsKaFsCstm");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        ctx.accounts.counter.bump = ctx.bumps.counter;
        Ok(())
    }

    pub fn increment(ctx: Context<Count>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }
    pub fn decrement(ctx: Context<Count>) -> Result<()> {
        ctx.accounts.counter.count -= 1;
        Ok(())
    
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer, 
        seeds = [
            b"counter",
            signer.key().as_ref(),
            ],
            bump,
            space = Counter::INIT_SPACE,
    )]
    counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Count<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [
            b"counter",
            signer.key().as_ref()
            ],
            bump = counter.bump,
    )]
    counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub bump: u8,
}

// impl = implementation
impl Space for Counter {
    const INIT_SPACE: usize = 8 + 8 + 1; // Anchor descriminator + counter + bump
}

