use super::setup::{get_vc_setup, VCVaultContractTest};
use crate::test::setup::VaultContractTest;
use soroban_sdk::{testutils::Address as _, vec, Address, String};

#[test]
fn test_initialize() {
    let VaultContractTest {
        env: _env,
        owner,
        issuer: _issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VaultContractTest {
        env: _,
        owner,
        issuer: _issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.initialize(&owner, &did_uri);
}

#[test]
fn test_authorize_issuer() {
    let VaultContractTest {
        env: _env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_authorize_issuer_with_already_authorized_issuer() {
    let VaultContractTest {
        env: _,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);
    contract.authorize_issuer(&owner, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_authorize_issuer_with_revoked_vault() {
    let VaultContractTest {
        env: _,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.revoke_vault(&owner);
    contract.authorize_issuer(&owner, &issuer);
}

#[test]
fn test_authorize_issuers() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    let issuers = vec![&env, issuer.clone()];
    contract.initialize(&owner, &did_uri);
    contract.authorize_issuers(&owner, &issuers);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_authorize_issuers_with_revoked_vault() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    let issuers = vec![&env, issuer.clone()];
    contract.initialize(&owner, &did_uri);
    contract.revoke_vault(&owner);
    contract.authorize_issuers(&owner, &issuers);
}

#[test]
fn test_revoke_issuer() {
    let VaultContractTest {
        env: _env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);
    contract.revoke_issuer(&owner, &issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_revoke_issuer_when_issuer_is_not_found() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);

    let invalid_issuer = Address::generate(&env);
    contract.revoke_issuer(&owner, &invalid_issuer);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_revoke_issuer_with_revoked_vault() {
    let VaultContractTest {
        env: _,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();
    contract.initialize(&owner, &did_uri);
    contract.revoke_vault(&owner);
    contract.revoke_issuer(&owner, &issuer);
}

#[test]
fn test_store_vc() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);
    contract.store_vc(
        &owner,
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_store_vc_with_empty_issuers() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&owner, &did_uri);
    contract.store_vc(
        &owner,
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_store_vc_with_issuer_not_found() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    let invalid_issuer = Address::generate(&env);

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);
    contract.store_vc(
        &owner,
        &vc_id,
        &vc_data,
        &invalid_issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_store_vc_with_revoked_issuer() {
    let VaultContractTest {
        env,
        owner,
        issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    let VCVaultContractTest {
        vc_id,
        vc_data,
        issuance_contract_address,
        issuer_did,
    } = get_vc_setup(&env);

    contract.initialize(&owner, &did_uri);
    contract.authorize_issuer(&owner, &issuer);
    contract.revoke_issuer(&owner, &issuer);

    contract.store_vc(
        &owner,
        &vc_id,
        &vc_data,
        &issuer,
        &issuer_did,
        &issuance_contract_address,
    )
}

#[test]
fn test_revoke_vault() {
    let VaultContractTest {
        env: _,
        owner,
        issuer: _,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&owner, &did_uri);
    contract.revoke_vault(&owner);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_migrate_should_fail_without_vcs() {
    let VaultContractTest {
        env: _,
        owner,
        issuer: _,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&owner, &did_uri);
    contract.migrate(&owner);
}

#[test]
fn test_set_admin() {
    let VaultContractTest {
        env,
        owner,
        issuer: _issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&owner, &did_uri);

    let new_admin = Address::generate(&env);

    contract.set_admin(&owner, &new_admin);
}

#[test]
fn test_version() {
    let VaultContractTest {
        env,
        owner,
        issuer: _issuer,
        did_uri,
        contract,
    } = VaultContractTest::setup();

    contract.initialize(&owner, &did_uri);
    let pkg_version = env!("CARGO_PKG_VERSION");
    let expected_version = String::from_str(&env, pkg_version);
    assert_eq!(contract.version(), expected_version)
}
