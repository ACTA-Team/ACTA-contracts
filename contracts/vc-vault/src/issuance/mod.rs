//! Issuance: VC status registry (valid/revoked/invalid).

use crate::error::ContractError;
use crate::model::VCStatus;
use crate::storage;
use soroban_sdk::{panic_with_error, Env, String};

pub fn revoke_vc(e: &Env, vc_id: String, date: String) {
    let vc_status = storage::read_vc_status(e, &vc_id);
    if vc_status != VCStatus::Valid {
        panic_with_error!(e, ContractError::VCAlreadyRevoked)
    }
    storage::write_vc_status(e, &vc_id, &VCStatus::Revoked(date))
}
