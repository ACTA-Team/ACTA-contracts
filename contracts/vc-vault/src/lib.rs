//! VC Vault Contract
//!
//! Soroban contract for Verifiable Credential storage and issuance status registry.
//! Provides per-owner vaults, issuer authorization, and VC lifecycle (issue, verify, revoke).

#![no_std]
#![allow(dead_code)]

mod api;
mod contract;
mod error;
mod issuance;
mod model;
mod storage;
mod vault;

#[cfg(test)]
mod test;
