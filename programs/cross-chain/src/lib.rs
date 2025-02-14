use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

#[program]
pub mod cross_chain {
    use super::*;

    // 
    pub fn engrave_to_bitcoin(ctx: Context<Engrave>, burn_proof: BurnProof) -> Result<()> {
        let inscription_data = format!(
            "PERPETU_DEATH_CERT|{}|{}|{}",
            burn_proof.total_burned,
            burn_proof.last_block,
            burn_proof.final_weights_hash
        );

        // 
        wormhole::post_message(
            ctx.accounts.wormhole_program.clone(),
            "bitcoin_mainnet",
            inscription_data.as_bytes(),
        )?;

        emit!(CrossChainEvent {
            chain: "Bitcoin".to_string(),
            operation: "OrdinalEngraving".to_string(),
            data_hash: sha256(inscription_data)
        });

        Ok(())
    }

    // 
    pub fn import_ethereum_memory(ctx: Context<ImportMemory>, eth_tx_hash: [u8; 32]) -> Result<()> {
        let eth_data = wormhole::receive_message(
            ctx.accounts.wormhole_program.clone(),
            "ethereum_mainnet",
            eth_tx_hash
        )?;

        // 
        let mut training_data = ctx.accounts.training_set.load_mut()?;
        training_data.push(TrainingSample {
            features: decode_eth_data(eth_data),
            label: "legacy".to_string(),
        });

        Ok(())
    }
}
