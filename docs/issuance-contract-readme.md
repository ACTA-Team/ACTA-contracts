# ACTA Issuance Contract — Guía y Uso

Esta guía describe cada función pública del contrato `acta_issuance_contract`, su modelo de autorización, cómo interactúa con el Vault y ejemplos de uso en testnet.

**IDs de Testnet**
- Issuance Contract: `CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA`
- Vault Contract: `CATIXW2QGZEBDOWK6HUPWR6OUDIXIRCAALUHBJHDNDBHF6WAHIC4VQZF`

## Conceptos Clave
- `admin`: dirección que controla el contrato de issuance. Debe firmar las acciones sensibles.
- `issuer_did`: DID del emisor guardado en el contrato para trazabilidad (p. ej., `did:pkh:stellar:testnet:G...`).
- `vc_id` y `vc_data`: identificador y payload cifrado de la credencial.
- `VCStatus`: estado de la VC en issuance: `valid`, `revoked(fecha)` o `invalid`.
- `vault_contract`: ID del contrato Vault del titular donde se almacena la VC.

## Reglas de Autorización y Estado
- Todas las acciones sensibles (`initialize`, `issue`, `revoke`, `migrate`, `upgrade`, `set_admin`) requieren que el `admin` actual firme (`require_auth`).
- `issue` invoca internamente `store_vc` en el Vault del titular, pasando al `admin` como `issuer`. El Vault debe:
  - estar inicializado para el `owner`;
  - tener autorizado al `issuer` (el `admin` del issuance);
  - no estar revocado.
- El contrato guarda localmente el estado de cada VC (`VCStatus`) para permitir `verify` y para evitar revocaciones duplicadas.

## Errores Comunes (códigos)
- `#1 AlreadyInitialized`: intentar inicializar un contrato que ya tiene `admin`.
- `#2 VCNotFound`: intentar revocar una VC inexistente/invalid.
- `#3 VCAlreadyRevoked`: intentar revocar una VC que ya está revocada.
- `#4 VCSAlreadyMigrated`: correr `migrate` cuando no hay VCs legacy que migrar.

---

## initialize
Inicializa el contrato de issuance con el `admin` y el `issuer_did`.

- Efecto: establece `admin` y guarda `issuer_did`.
- Requisitos: falla con `#1` si ya está inicializado.
- Uso típico: preparar el contrato para emitir VCs.

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet --source acta_sc_source -- initialize --admin G...ADMIN --issuer_did did:pkh:stellar:testnet:G...ADMIN`

---

## issue
Emite una VC y la almacena en el Vault del titular.

- Efecto:
  - Invoca `store_vc(owner, vc_id, vc_data, issuer, issuer_did, issuance_contract)` en el Vault.
  - Guarda el estado local `VCStatus::Valid` para `vc_id` en issuance.
  - Devuelve el `vc_id`.
- Requisitos:
  - Debe firmar el `admin` del issuance.
  - El Vault del titular debe tener al `admin` autorizado como `issuer`.
- Uso típico: flujos de emisión desde backend para uno o varios titulares.

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet --source acta_sc_source -- issue --owner G...OWNER --vc_id "vc-123" --vc_data "<encrypted_payload>" --vault_contract CATIXW2QGZEBDOWK6HUPWR6OUDIXIRCAALUHBJHDNDBHF6WAHIC4VQZF`

---

## verify
Verifica el estado de una VC.

- Efecto: retorna un `Map{ "status": "valid"|"invalid"|"revoked", "since": fecha? }`.
- Requisitos: ninguno (solo lectura).
- Uso típico: comprobaciones rápidas de estado (p. ej., en verificación de acceso).

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet -- verify --vc_id vc-123`

---

## revoke
Revoca una VC por `vc_id`, registrando la `date` de revocación.

- Efecto: cambia el estado a `VCStatus::Revoked(date)`.
- Requisitos:
  - Debe firmar el `admin`.
  - Falla con `#2` si la VC no existe/invalid.
  - Falla con `#3` si la VC ya estaba revocada.
- Uso típico: retirar una VC por cumplimiento, fraude o expiración.

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet --source acta_sc_source -- revoke --vc_id vc-123 --date 2024-10-15T12:00:00Z`

---

## migrate
Migra almacenamiento legacy (`VCs`, `Revocations`) al esquema actual `VCStatus` por clave.

- Efecto: re‑almacena VCs legacy y elimina las claves antiguas.
- Requisitos: debe firmar el `admin`; falla con `#4` si no hay VCs legacy.
- Uso típico: ejecutar una vez tras un upgrade que cambió el esquema.

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet --source acta_sc_source -- migrate`

---

## set_admin
Cambia el `admin` del contrato de issuance.

- Efecto: asigna `new_admin` como administrador.
- Requisitos: debe firmar el `admin` actual.
- Uso típico: trasladar control a otra entidad (custodio, multisig, etc.).

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet --source acta_sc_source -- set_admin --new_admin G...NEWADMIN`

---

## upgrade
Actualiza el código WASM del contrato.

- Efecto: reemplaza el WASM con el hash `new_wasm_hash` (32 bytes).
- Requisitos: debe firmar el `admin`.
- Uso típico: desplegar nuevas versiones con mejoras o correcciones.

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet --source acta_sc_source -- upgrade --new_wasm_hash a7a34e34b16d6ad3d4876f58737dfcbbaa8b5bb21abe17a3da022d181e4da3917`

---

## version
Devuelve la versión del paquete (`CARGO_PKG_VERSION`) compilada en el contrato.

- Uso típico: auditar qué versión está ejecutándose en red.

Ejemplo (testnet):
`soroban contract invoke --id CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA --network testnet -- version`

---

## Visibilidad de Datos y Privacidad
- El estado en Soroban es público: las reglas de autorización controlan ejecución/escritura, no lectura cruda.
- No almacenes PII en claro: cifra `vc_data` fuera de cadena; considera almacenar solo `sha256(vc_data)` y metadatos mínimos.
- `issuer_did` y `issuance_contract` (en Vault) son metadatos de trazabilidad.

## Interacción con Vault
- El flujo recomendado:
  - Inicializa el Vault del `owner` y registra su `did_uri`.
  - Autoriza al `admin` del issuance como `issuer` en ese Vault.
  - Llama a `issue` en issuance: esto guarda la VC en el Vault y marca estado `valid` en issuance.
- Para revocación, usa `revoke` en issuance; `verify` consultará el estado local.

## Buenas Prácticas
- Alinear firmante con rol: `admin` firma administración; `issuer` (admin del issuance) queda autorizado en cada Vault.
- Gestionar IDs en configuración (p. ej., `.env`):
  - `ISSUANCE_CONTRACT_ID=CBRG5UJ7JZRIQMO2LCOUGODWXPSIXD2H5EMCCUP5BWZOKJ73AHNH4RUA`
  - `VAULT_CONTRACT_ID=CATIXW2QGZEBDOWK6HUPWR6OUDIXIRCAALUHBJHDNDBHF6WAHIC4VQZF`