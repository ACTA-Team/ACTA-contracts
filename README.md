# ACTA Smart Contracts

Emisión, almacenamiento y verificación de credenciales verificables (VC) sobre Soroban.

Este monorepo contiene los contratos ACTA:
- `ACTA Issuance`: emisión, verificación y revocación de VCs.
- `ACTA Vault`: almacenamiento cifrado y autorización de emisores.
- `Deployer`: despliegue atómico de contratos y su inicialización.
- `DID` (opcional): contrato DID W3C (se compila/instala por separado).

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

El script imprime los WASM IDs y la dirección del Deployer.

## Deployed Contract IDs (Testnet)

- Issuance: `CC5VJEF2D56G5C3JL6JKGS5T3NNHTE7LCLSK26CMX55P55F3GP365KN6`
  - Explorer: https://stellar.expert/explorer/testnet/contract/CC5VJEF2D56G5C3JL6JKGS5T3NNHTE7LCLSK26CMX55P55F3GP365KN6
- Vault: `CCC3DH5D3P2VP2OXCFYSC25E6I7SKHKPH3GLISJIR3IMZ3W2STKTESIE`
  - Explorer: https://stellar.expert/explorer/testnet/contract/CCC3DH5D3P2VP2OXCFYSC25E6I7SKHKPH3GLISJIR3IMZ3W2STKTESIE
- DID: `CBD2UKHTCYDHDGKLFZ47NOFXSFVTWBA6GADR5EYHPIHGXBW6RKOGTB4V`
  - Explorer: https://stellar.expert/explorer/testnet/contract/CBD2UKHTCYDHDGKLFZ47NOFXSFVTWBA6GADR5EYHPIHGXBW6RKOGTB4V
- Deployer: `CDIBXZUIZBQG7IMFFCMTSIKTL6JCPSVENHHZQ6CUXD5E26WNF6PVVQLK`
  - Explorer: https://stellar.expert/explorer/testnet/contract/CDIBXZUIZBQG7IMFFCMTSIKTL6JCPSVENHHZQ6CUXD5E26WNF6PVVQLK

## Licencia
Este software está licenciado bajo [Apache License 2.0](./LICENSE).
