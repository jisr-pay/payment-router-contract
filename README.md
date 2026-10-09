<p align="center"><img src="https://raw.githubusercontent.com/jisr-pay/.github/main/assets/icon.svg" alt="Jisr" width="72"></p>

# Payment Router Contract

Deployment inspection and original-source recovery workspace for Jisr Pay's Soroban payment router. **Original Rust source is not present.** No replacement contract is represented as the deployed original.

[deployment-reference.json](deployment-reference.json) records the current web client's Testnet defaults and expected invocation. These defaults do not prove that the contract is currently available, what fees it charges, or how it authorizes transfers. The existing repository license is retained; the recovered source's licensing and provenance still need checking.

## Read-only inspection

Use Node 24.15+ within Node 24:

```powershell
npm ci
npm run build
npm run inspect
```

Inspection verifies the RPC network passphrase, fetches deployed WASM by contract ID, computes its SHA-256 and lists module exports. It writes `artifacts/deployed.wasm` and `artifacts/inspection.json` locally; these generated files are ignored. It performs no signing, deployment or transaction submission. Network errors or missing/expired Testnet state cause the command to fail, not fabricate an artifact. Previous inspection files retain their timestamps and must not be interpreted as a fresh successful check.

WASM retrieval does not recover the original Rust source or establish source correspondence. Stellar also documents fetching contract bytecode with its CLI: https://developers.stellar.org/docs/build/guides/dapps/working-with-contract-specs

## Needed from the original source holder

- Original source repository or Rust source files, Cargo.toml and Cargo.lock.
- Exact source commit and license/provenance.
- Rust, Stellar CLI and Soroban SDK versions and reproducible build command.
- Deployment transaction hash, ledger/network, WASM hash and any upgrade history.
- Tests and expected fee, authorization and transfer behavior.

Once source is recovered, build it reproducibly and compare its WASM hash with the retrieved ledger artifact. Establish the exact route_payment argument and result specification, authorization requirements, token handling, treasury fee rounding and failure behavior. Add tests for unauthorized senders, invalid amounts, fee boundaries, insufficient balances and atomic failure. Do not deploy or upgrade as part of source recovery.
