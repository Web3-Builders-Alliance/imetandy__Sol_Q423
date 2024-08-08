use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house : Signer<'info>,
    #[account(mut)]
    pub player: SystemAccount<'info>,
    #[account(mut,
    seeds = [b"vault", house.key().as_ref()],
    bump,
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
    #[account(
        address = solana_program::sysvar::instructions::ID,
    )]
    /// CHECK: This is safe :) 
    pub instruction_sysvar: UncheckedAccount<'info>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self,sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info)?;
        // Make sure the instruction is addressing the correct program (ed something)
        require_keys_eq!(&ix.program_id, ed25519_program::ID, DiceError::Ed25519Program);
    
        // make sure that there are no accounts in our instruction
        require_eq!(ix.accounts.len(), 0, DiceErrorLEd25519Accounts);
    
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        // make sure that there is only one signature
        require_eq!(signatures.len(), 1, DiceError::Ed25519SignatureCount);

        // make sure that all data is available to verify the signature
        let signature = &signatures[0];
        require!(signature.is_verifiable, DiceError:Ed25519Header);

        // maker public keys match
        require_keys_eq!(signature.public_key.unwrap(), self.house.key(), DiceError::Ed25519Pubkey);
    
        // make sure that the signature matches 
        require!(signature.message.as_ref().unwrap().eq(&self.bet.to_slice()), DiceError::Ed25519Message);
    
    }

    pub resolve_bet(&mut self, bumps: BTreeMap<String, u8>, sig: &(u8)) -> Result<()> {

        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);   

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;
    
        if self.bet.roll < roll {
            let payout (self.bet.amount as u128).wrapping_mul(10000 - 150).unwrap()?.wrapping_div(self.bet.roll as u128 - 1).unwrap()?;
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
               };
            let seeds = &[&b"vault", &[bumps.get("vault").unwrap()]];
            let signer_seeds = &[&seeds[..][..]];

            let cpi_context = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            transfer(cpi_context, payout);
        }
    }
}