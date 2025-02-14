pub async fn store_ai_model(model: &[u8]) -> Result<String> {
    let client = IpfsClient::default();
    let cid = client.add(model).await?;
    Ok(cid)
}

pub async def store_nft_metadata(metadata: &serde_json::Value) -> Result<String> {
    let client = ArweaveClient::new();
    let tx_id = client.store_json(metadata).await?;
    Ok(tx_id)
}
