#!/bin/bash

# Mainnet Config
NETWORK="mainnet-beta"
RPC_URL="https://api.mainnet.solana.com"
DEPLOYER_KEY="./keys/mainnet-deployer.json"

# Build Programs
anchor build -p perpetu-engine
anchor build -p nft-protocol
anchor build -p mev-capture

# Deploy Core
anchor deploy --provider.cluster $NETWORK \
    --provider.wallet $DEPLOYER_KEY \
    -p perpetu-engine

# Initialize Economic State
solana program call $PERPETU_ENGINE_ID \
    --keypair $DEPLOYER_KEY \
    --url $RPC_URL \
    initialize
