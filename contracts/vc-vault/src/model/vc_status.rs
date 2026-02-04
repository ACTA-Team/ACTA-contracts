use soroban_sdk::{contracttype, String};

/// Status registry entry for a VC ID.
#[derive(PartialEq)]
#[contracttype]
pub enum VCStatus {
    /// VC exists and is currently valid.
    Valid,

    /// VC does not exist in the registry.
    Invalid,

    /// VC was revoked at the given ISO-8601 date string.
    Revoked(String),
}
