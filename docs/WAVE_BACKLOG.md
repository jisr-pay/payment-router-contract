# Wave engineering backlog

Drafted October 8, 2026 against current implementation. These are proposed contributor tasks, not Wave enrollment or earned points. Complexity requires maintainer review in the app.

## 1. Add a reviewed instance TTL and restoration workflow

## Context

Define threshold/extension behavior and document restoration; cover near-expiry invocation and archived-instance failure without changing fee policy.

## Acceptance criteria

- Implement and document the specific behavior above.
- Cover positive, negative and unavailable-input cases appropriate to the change.
- Pass the repository documented build/test checks and required CI.
- Preserve exact identities/amounts and uncertain evidence outcomes.

## Relevant files

src/lib.rs

## Proposed complexity

medium; planning only. Actual Wave complexity and enrollment are set by maintainers in the Drips app.

## Contribution

Open a focused feat/fix/test/docs branch. PRs explain behavior and actual validation and include Closes #<issue_id>. Follow CONTRIBUTING.md and SECURITY.md.

## 2. Decode router settlement evidence in the Jisr API

## Context

Specify the routed event ABI and coordinate consumer checks for sender, recipient, token, gross/net/fee and contract identity; reject unrelated or incomplete events.

## Acceptance criteria

- Implement and document the specific behavior above.
- Cover positive, negative and unavailable-input cases appropriate to the change.
- Pass the repository documented build/test checks and required CI.
- Preserve exact identities/amounts and uncertain evidence outcomes.

## Relevant files

src/lib.rs, ../jisr-api/src/

## Proposed complexity

high; planning only. Actual Wave complexity and enrollment are set by maintainers in the Drips app.

## Contribution

Open a focused feat/fix/test/docs branch. PRs explain behavior and actual validation and include Closes #<issue_id>. Follow CONTRIBUTING.md and SECURITY.md.

## 3. Verify a consumer invocation against deployed WASM provenance

## Context

Record build revision, installed WASM hash, constructor policy and SDK invocation; prove a changed policy or wrong contract fails.

## Acceptance criteria

- Implement and document the specific behavior above.
- Cover positive, negative and unavailable-input cases appropriate to the change.
- Pass the repository documented build/test checks and required CI.
- Preserve exact identities/amounts and uncertain evidence outcomes.

## Relevant files

scripts/testnet-demo.py, docs/

## Proposed complexity

medium; planning only. Actual Wave complexity and enrollment are set by maintainers in the Drips app.

## Contribution

Open a focused feat/fix/test/docs branch. PRs explain behavior and actual validation and include Closes #<issue_id>. Follow CONTRIBUTING.md and SECURITY.md.

## 4. Add token-transfer denial and rollback fixtures

## Context

Exercise an issuer-controlled token that denies the fee transfer after the recipient transfer; confirm both balances roll back and no successful settlement is reported.

## Acceptance criteria

- Implement and document the specific behavior above.
- Cover positive, negative and unavailable-input cases appropriate to the change.
- Pass the repository documented build/test checks and required CI.
- Preserve exact identities/amounts and uncertain evidence outcomes.

## Relevant files

src/test.rs

## Proposed complexity

medium; planning only. Actual Wave complexity and enrollment are set by maintainers in the Drips app.

## Contribution

Open a focused feat/fix/test/docs branch. PRs explain behavior and actual validation and include Closes #<issue_id>. Follow CONTRIBUTING.md and SECURITY.md.

## 5. Document integer fee rounding with consumer examples

## Context

Show tiny, ordinary and maximum amounts, gross debit versus recipient credit, and the 1000-bps policy cap; derive examples from executable calculations.

## Acceptance criteria

- Implement and document the specific behavior above.
- Cover positive, negative and unavailable-input cases appropriate to the change.
- Pass the repository documented build/test checks and required CI.
- Preserve exact identities/amounts and uncertain evidence outcomes.

## Relevant files

README.md, docs/

## Proposed complexity

trivial; planning only. Actual Wave complexity and enrollment are set by maintainers in the Drips app.

## Contribution

Open a focused feat/fix/test/docs branch. PRs explain behavior and actual validation and include Closes #<issue_id>. Follow CONTRIBUTING.md and SECURITY.md.

## 6. Design reviewed deployment ownership and network release gates

## Context

Specify operational treasury ownership, source/WASM verification and Testnet-to-Mainnet gates; keep the disposable Testnet treasury out of production configuration.

## Acceptance criteria

- Implement and document the specific behavior above.
- Cover positive, negative and unavailable-input cases appropriate to the change.
- Pass the repository documented build/test checks and required CI.
- Preserve exact identities/amounts and uncertain evidence outcomes.

## Relevant files

docs/, scripts/

## Proposed complexity

high; planning only. Actual Wave complexity and enrollment are set by maintainers in the Drips app.

## Contribution

Open a focused feat/fix/test/docs branch. PRs explain behavior and actual validation and include Closes #<issue_id>. Follow CONTRIBUTING.md and SECURITY.md.

## Published issue links

- [Add a reviewed instance TTL and restoration workflow](https://github.com/jisr-pay/payment-router-contract/issues/2)
- [Decode router settlement evidence in the Jisr API](https://github.com/jisr-pay/payment-router-contract/issues/3)
- [Verify a consumer invocation against deployed WASM provenance](https://github.com/jisr-pay/payment-router-contract/issues/4)
- [Add token-transfer denial and rollback fixtures](https://github.com/jisr-pay/payment-router-contract/issues/5)
- [Document integer fee rounding with consumer examples](https://github.com/jisr-pay/payment-router-contract/issues/6)
- [Design reviewed deployment ownership and network release gates](https://github.com/jisr-pay/payment-router-contract/issues/7)
