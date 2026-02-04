use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    IssuerNotAuthorized = 2,
    IssuerAlreadyAuthorized = 3,
    VaultRevoked = 4,
    VCSAlreadyMigrated = 5,
    VCNotFound = 6,
    VCAlreadyRevoked = 7,
    VaultNotInitialized = 8,
    NotInitialized = 9,
    InvalidVaultContract = 10,
}
