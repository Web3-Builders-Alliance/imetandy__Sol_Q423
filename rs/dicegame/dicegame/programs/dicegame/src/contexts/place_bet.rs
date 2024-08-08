use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instuction(seed: u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: systemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space: Bet::INIT_SPACE,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(&mut self, bumps: &BTreeMap<String, u8>, seed: u128, roll: u8, amount: u64) -> Result<()> {
        self.bet.solt = Clock::get()?.slot;
        self.bet.player = self.player.key();
        self.bet.seed = seed;   
        self.bet.roll = roll;
        self.bet.amount = amount;
        self.bet.bump = *bumps.get("bet").unwrap();
    }
    pub fn deposit(&mut self, amount:u64) -> Result<()> {
    let account = Transfer {
        from: self.player.to_account_info(),
        to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), account);

        transfer(cpi_ctx, amount);
    }
}

