#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub nft_account: Account<'info, NFTSate>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NFTState {
    pub tx_hash: [u8; 32],       // Original failed transaction
    pub layers: Vec<String>,     // Satirical text layers
    pub royalties: u64,          // Percentage basis points
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NFTMetadata {
    pub existential_question: String,
    pub trading_history: Vec<Pubkey>,
}

impl NFTState {
    pub fn add_satire_layer(&mut self, new_text: String) {
        self.layers.push(new_text);
        self.royalties += 50;  // 0.5% increase per layer
    }
}
