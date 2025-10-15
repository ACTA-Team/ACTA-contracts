use crate::verifiable_credential::VerifiableCredential;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ContractAdmin,         // Address
    Admin(Address),        // Address
    Did(Address),          // String
    Revoked(Address),      // Boolean
    Issuers(Address),      // Vec<Address>
    VC(Address, String),   // VerifiableCredential
    VCs(Address),          // Vec<VerifiableCredential>
}

pub fn has_admin(e: &Env, owner: &Address) -> bool {
    let key = DataKey::Admin(owner.clone());
    e.storage().instance().has(&key)
}

pub fn has_contract_admin(e: &Env) -> bool {
    let key = DataKey::ContractAdmin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env, owner: &Address) -> Address {
    let key = DataKey::Admin(owner.clone());
    e.storage().instance().get(&key).unwrap()
}

pub fn read_contract_admin(e: &Env) -> Address {
    let key = DataKey::ContractAdmin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, owner: &Address, admin: &Address) {
    let key = DataKey::Admin(owner.clone());
    e.storage().instance().set(&key, admin);
}

pub fn write_contract_admin(e: &Env, admin: &Address) {
    let key = DataKey::ContractAdmin;
    e.storage().instance().set(&key, admin);
}

pub fn write_did(e: &Env, owner: &Address, did: &String) {
    let key = DataKey::Did(owner.clone());
    e.storage().instance().set(&key, did);
}

// DID generativo: el address del contrato DID ya no se guarda.

pub fn read_revoked(e: &Env, owner: &Address) -> bool {
    let key = DataKey::Revoked(owner.clone());
    e.storage().instance().get(&key).unwrap()
}

pub fn write_revoked(e: &Env, owner: &Address, revoked: &bool) {
    let key = DataKey::Revoked(owner.clone());
    e.storage().instance().set(&key, revoked);
}

pub fn read_issuers(e: &Env, owner: &Address) -> Vec<Address> {
    let key = DataKey::Issuers(owner.clone());
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_issuers(e: &Env, owner: &Address, issuers: &Vec<Address>) {
    let key = DataKey::Issuers(owner.clone());
    e.storage().persistent().set(&key, issuers)
}

pub fn write_vc(e: &Env, owner: &Address, vc_id: &String, vc: &VerifiableCredential) {
    let key = DataKey::VC(owner.clone(), vc_id.clone());
    e.storage().persistent().set(&key, vc)
}

pub fn read_old_vcs(e: &Env, owner: &Address) -> Option<Vec<VerifiableCredential>> {
    let key = DataKey::VCs(owner.clone());
    e.storage().persistent().get(&key)
}

pub fn remove_old_vcs(e: &Env, owner: &Address) {
    let key = DataKey::VCs(owner.clone());
    e.storage().persistent().remove(&key);
}
