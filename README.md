# ACTA Smart Contracts

Issuance, storage, and verification of Verifiable Credentials (VC) on Soroban.

This monorepo contains the ACTA contracts:
- `ACTA Issuance`: issue, verify, and revoke VCs.
- `ACTA Vault`: multi-tenant encrypted storage and issuer authorization per owner.
 - (Optional) Off-chain DID resolution: the vault stores a DID URI string per owner (no DID contract deployed).

## Build

```bash
chmod +x build.sh
sh build.sh
```

## Release (Testnet)

```bash
chmod +x release.sh
sh release.sh
```

## Deployed Contract IDs (Testnet)

Note: These are example IDs from test deployments. Replace with your own as needed.

- Issuance: `CAULJ65QZR4FCHAZGBUHMDACT7PODYIE54HGGOQWJRQFATAJ4U2HOUQK`
  - Explorer: https://stellar.expert/explorer/testnet/contract/CAULJ65QZR4FCHAZGBUHMDACT7PODYIE54HGGOQWJRQFATAJ4U2HOUQK
- Vault: `CCDAKJJROTWOEQS42TULG23YSM2OLGFKK43OQ3FRL6TQWQCC3QX4EUDH`
  - Explorer: https://stellar.expert/explorer/testnet/contract/CCDAKJJROTWOEQS42TULG23YSM2OLGFKK43OQ3FRL6TQWQCC3QX4EUDH

## License
This software is licensed under the [Apache License 2.0](./LICENSE).
