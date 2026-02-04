#!/bin/sh
# Config testnet in local.
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" || true

# Generate key to sign the transactions.
soroban keys generate vc_vault_admin --network testnet || true

# Build + optimize
sh scripts/build.sh

echo "VC Vault contract ID:"
soroban contract deploy \
  --wasm target/wasm32v1-none/release/vc_vault_contract.optimized.wasm \
  --source vc_vault_admin \
  --network testnet
