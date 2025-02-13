#[tokio::test]
async fn test_quantum_rule_switch() {
    let mut test_env = TestEnvironment::new().await;
    
    // Initial state
    let init_state = test_env.get_economic_state().await;
    assert_eq!(init_state.tax_rates["buy"], 0.05);

    // Trigger bull market prediction
    test_env.simulate_prediction(PredictionData {
        bull_prob: I80F48::from_num(0.65),
        confidence: I80F48::from_num(0.9),
        last_weights_hash: [0;32]
    }).await;

    // Verify FOMO mode activation
    let new_state = test_env.get_economic_state().await;
    assert_eq!(new_state.tax_rates["buy"], 0.03);
    assert_eq!(new_state.tax_rates["sell"], 0.10);
    
    // Test MEV redistribution
    let mev_profit = 100_000_000; // 100 SOL
    test_env.simulate_mev_capture(mev_profit).await;
    let burn_amount = mev_profit * 70 / 100;
    assert_eq!(test_env.get_burn_total().await, burn_amount);
}
