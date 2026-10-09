# Atomic authorized token payments on Stellar

Jisr payment router implements immutable token/treasury/fee policy and sender-authorized atomic token transfers. It prevents substituted policy parameters, rounds basis-point fees using integer arithmetic, and rolls back recipient transfers if the fee transfer fails.

October 8 validation: six Soroban tests, cargo check, formatting, and release WASM build passed on Rust 1.96.0 / SDK 27.0.6. A fresh synthetic Testnet deployment and authorized 1 XLM payment succeeded; recipient and treasury credits were verified as 0.9875 and 0.0125 XLM.

- [Contract](https://stellar.expert/explorer/testnet/contract/CCGSUUQLWXKU6AZ6YKUNXLR7R6KLBYBG4AJGJ54XV4DC63AJ3LDVPNW4)
- [Deployment](https://stellar.expert/explorer/testnet/tx/2b64055454053e2b0fb6c404755c078fb457419616fcab0ce20906e408e13d9c)
- [Payment](https://stellar.expert/explorer/testnet/tx/6e79ed7847d34d19fb0d9f8bf43282cd583972537a41d539559a610a7f108910)
- [Machine-readable evidence](testnet-result-2026-10-08.json)

Build/test instructions and the repeatable live demo are in README.md. The five payment arguments match the Jisr SDK ABI. This new source does not establish provenance for older deployments. API contract-event verification, browser acceptance, TTL operations, escrow and Mainnet remain separate work. No adoption or security-audit claim is made.

Maintainers: xteesamz and EthTobi, owner-confirmed; GitHub contact, anytime availability. Review the implementation PR, confirm required CI and App coverage, then apply through the Drips maintainer dashboard using the reviewed revision. Backlog complexity is proposed until app enrollment.
