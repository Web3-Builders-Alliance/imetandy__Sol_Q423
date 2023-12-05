use anchor_lang::prelude::*;

declare_id!("DPXqfPv4i5NkU4ENmgQLAfD2LWKpYLyzxFFsKaFsCstm");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, count_value: u64) -> Result<()> {
        ctx.accounts.counter.count = count_value;
        ctx.accounts.counter.bump = ctx.bumps.counter;
        msg!("Counter Initialized");
        Ok(())
    }

    pub fn increment(ctx: Context<Count>, count_value: u64) -> Result<()> {
        ctx.accounts.counter.count += count_value;
        msg!("Counter Increased to: {}", ctx.accounts.counter.count);
        Ok(())
    }
    pub fn decrement(ctx: Context<Count>, count_value: u64) -> Result<()> {
        ctx.accounts.counter.count -= count_value;
        msg!("Counter Decreased to: {}", ctx.accounts.counter.count);
        Ok(())
    
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
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
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Count<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [
            b"counter",
            signer.key().as_ref()
            ],
            bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
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

