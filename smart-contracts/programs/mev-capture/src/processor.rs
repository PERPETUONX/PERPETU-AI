pub fn detect_and_penalize(ctx: Context<DetectMEV>) -> Result<()> {
    let recent_slots = (0..5)
        .map(|i| ctx.accounts.history[i])
        .collect::<Vec<SlotHistory>>();

    let mev_pattern = analyze_sandwich_pattern(
        recent_slots,
        ctx.accounts.current_tx.clone()
    );

    if mev_pattern.confidence > 0.8 {
        let penalty = calculate_mev_penalty(
            mev_pattern.profit_estimate,
            ctx.accounts.offender.lamports()
        );
        
        invoke_signed(
            &system_instruction::transfer(
                &ctx.accounts.offender.key(),
                &ctx.accounts.protocol_vault.key(),
                penalty,
            ),
            &[
                ctx.accounts.offender.clone(),
                ctx.accounts.protocol_vault.clone(),
            ],
            &[],
        )?;

        emit!(MEVPenaltyEvent {
            offender: ctx.accounts.offender.key(),
            penalty_amount: penalty,
            detection_model_version: MODEL_VERSION
        });
    }
    
    Ok(())
}
