# Contributing to GENOS

GENOS is built in public, one task per day. Every contribution that ships
to users generates automatic USDC rewards via the GENOS treasury smart
contract on Base (Ethereum L2).

---

## Rewards

| Contribution | Reward | Vesting |
|---|---|---|
| Merged feature PR | $200–1,000 USDC | 90 days |
| Critical bug fix | $500–2,000 USDC | Immediate |
| Provider manifest published | $50 USDC + 5% revenue share | Forever |
| New approved agent | $300–800 USDC + 3% of Pro subscriptions | 90 days |
| Documentation PR | $20–100 USDC | Immediate |

Revenue share is permanent. If you publish a provider manifest and 10,000
users use that provider, you receive 5% of every subscription that touches
it. Forever. On-chain. No intermediary.

---

## How to contribute

1. Fork the repository
2. Create a branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Open a PR with a clear description of what and why
6. Add your Base wallet address to the PR description for USDC payment

---

## Code standards

- Language: Rust for all system components
- Format: `cargo fmt` before every commit
- Lint: `cargo clippy` with zero warnings
- Tests: every public function needs at least one test
- Commits: conventional commits (feat/fix/docs/chore/refactor)

---

## Architecture principles

- AI as system primitive, not application
- Every agent action is reversible (protocol CARA)
- Capability-based security — least privilege always
- 100% local by default, cloud opt-in explicit
- Privacy by architecture, not by policy

---

## License

By contributing you agree your code is released under MIT license.
