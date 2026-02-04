use soroban_sdk::{contracttype, Address, String};

/// Verifiable Credential stored in a vault.
///
/// `data` is expected to be **ciphertext** (encrypted off-chain) or a safe reference.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableCredential {
    /// Application-level VC identifier.
    pub id: String,

    /// VC payload (ciphertext or reference).
    pub data: String,

    /// Issuance contract that can verify/revoke the VC status.
    pub issuance_contract: Address,

    /// Issuer DID (metadata for wallets/UX).
    pub issuer_did: String,
}
