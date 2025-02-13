#[event]
pub struct MEVEvent {
    pub profit: u64,
    pub attacker: Pubkey,
    pub redistributed: u64,
}

pub fn capture_mev(ctx: Context<CaptureMEV>, mev_profit: u64) -> Result<()> {
    let burn_amount = mev_profit * 70 / 100;
    let redistribution = mev_profit * 30 / 100;
    
    // Burn logic
    perpetual_token::burn(
        ctx.accounts.token_program.clone(),
        burn_amount,
    )?;

    // Distribute to SBT holders
    distribute_to_sbt_holders(redistribution)?;

    emit!(MEVEvent {
        profit: mev_profit,
        attacker: ctx.accounts.attacker.key(),
        redistributed: redistribution
    });
    
    Ok(())
}
