#[derive(Accounts)]
pub struct AntiSybil<'info> {
    #[account(seeds = [b"identity", user.key().as_ref()], bump)]
    pub identity: Account<'info, SybilResistance>,
    #[account(mut)]
    pub user: Signer<'info>,
}

impl AntiSybil {
    pub fn check_identity(&self) -> Result<()> {
        let current_epoch = Clock::get()?.epoch;
        require!(
            self.identity.last_interaction < current_epoch - 2,
            SybilError::TooFrequent
        );
        self.identity.last_interaction = current_epoch;
        Ok(())
    }
}

#[error_code]
pub enum SybilError {
    #[msg("Too frequent interactions detected")]
    TooFrequent,
    #[msg("Soulbound token already minted")]
    DuplicateSoul,
}
