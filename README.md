# Jisr payment router

A Soroban contract that atomically splits an authorized token payment between a recipient and a configured platform treasury. The immutable constructor policy fixes the token and fee (0–1000 basis points). `route_payment` requires sender authorization, checks the supplied token/treasury against policy, rejects invalid amounts and recipients, and emits a routed event. Amount is the total sender debit in integer token units; fees round down.

## Build and verify

Rust 1.96+, Soroban SDK 27.0.6, and the `wasm32v1-none` target:

```sh
cargo fmt --check
cargo check --locked
cargo test --locked --all
cargo build --locked --target wasm32v1-none --release
```

Tests cover exact splits, missing/wrong authorization, rollback after the fee transfer fails, policy mismatch, invalid amounts, maximum i128 arithmetic and tiny-fee rounding.

## October 8 Testnet demonstration

- Router: [`CCGSUUQLWXKU6AZ6YKUNXLR7R6KLBYBG4AJGJ54XV4DC63AJ3LDVPNW4`](https://stellar.expert/explorer/testnet/contract/CCGSUUQLWXKU6AZ6YKUNXLR7R6KLBYBG4AJGJ54XV4DC63AJ3LDVPNW4).
- Deployment: [successful transaction](https://stellar.expert/explorer/testnet/tx/2b64055454053e2b0fb6c404755c078fb457419616fcab0ce20906e408e13d9c).
- Authorized payment: [successful transaction](https://stellar.expert/explorer/testnet/tx/6e79ed7847d34d19fb0d9f8bf43282cd583972537a41d539559a610a7f108910).
- 1 XLM total debit; 0.9875 XLM recipient credit and 0.0125 XLM treasury credit verified against Horizon balances.

[The evidence record](docs/testnet-result-2026-10-08.json) includes source WASM SHA-256, accounts, amounts, ledger numbers and transaction hashes. `python3 scripts/testnet-demo.py` repeats the live synthetic demo with disposable keys and funded Testnet accounts; it requires Stellar CLI 28 and the built WASM. No live network work runs in CI.

## Integration and scope

The five `route_payment` arguments match the existing Jisr SDK: sender, recipient, platform treasury, token address and amount. This is a new implementation and deployment; it is not recovered provenance for an older configured router. Configure its actual policy in a consumer before invoking it. Jisr API currently verifies native payments only and leaves contract claims unverified. Browser signing, API contract-event verification and Mainnet acceptance remain separate release work.

This contract supports atomic token transfer plus a fee; no escrow, exchange routing, provider settlement, upgrades or administration is implemented. Instance storage follows Stellar TTL; operators must manage archival/restoration. The disposable demo treasury is a synthetic test fixture, not an operational account.

See [submission brief](docs/SUBMISSION.md), [backlog](docs/WAVE_BACKLOG.md), and the organization [governance files](https://github.com/jisr-pay/.github). Use focused branches and include `Closes #<issue_id>` in implementation PRs. Report vulnerabilities through GitHub private advisories; keep credentials out of repositories and logs.
