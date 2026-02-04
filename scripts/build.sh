#!/bin/sh
soroban contract build
# Soroban SDK/CLI v21 outputs WASM under wasm32v1-none by default.
soroban contract optimize --wasm target/wasm32v1-none/release/vc_vault_contract.wasm
