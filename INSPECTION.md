# Testnet deployment observation

Read-only inspection completed at **2026-09-11T21:37:07.127Z**.

- RPC: `https://soroban-testnet.stellar.org`
- Confirmed network passphrase: `Test SDF Network ; September 2015`
- Contract: `CDNQ7OMHIFOLZHOKWQLOGDW7CF3DRMKXJC6OULNGNBWF4O4NO2NEIGER`
- Retrieved WASM size: **3,401 bytes**
- WASM SHA-256: `ab6715d3611c45b0e2c7764e496635e28f141adb245392ac74bb80325c7164c2`
- Function export present: **`route_payment`**
- Contract specification custom sections: **1**

The retrieved contract specification declares `route_payment(sender: Address, recipient: Address, platform_treasury: Address, token_address: Address, amount: i128)` with no declared return values. This matches the client's argument order. It does not describe authorization or fee behavior.

This confirms that the configured contract's bytecode was retrievable on Testnet at the observation time and exported the function invoked by the client. It does not establish original source correspondence, authorization correctness, fee behavior or future availability. No transaction was submitted.

Run `npm run inspect` to reproduce a fresh observation. Local raw output is in `artifacts/inspection.json` and `artifacts/deployed.wasm`; generated artifacts are ignored by Git. Original source and reproducible-build information are still needed.
