//! Store VC payload in vault and update index.

use crate::model::VerifiableCredential;
use crate::storage;
use soroban_sdk::{Address, Env, String};

/// Write VC to vault and append ID to index.
pub fn store_vc(
    e: &Env,
    owner: &Address,
    id: String,
    data: String,
    issuance_contract: Address,
    issuer_did: String,
) {
    let new_vc = VerifiableCredential {
        id: id.clone(),
        data,
        issuance_contract,
        issuer_did,
    };
    storage::write_vault_vc(e, owner, &id, &new_vc);
    storage::append_vault_vc_id(e, owner, &id);
}
