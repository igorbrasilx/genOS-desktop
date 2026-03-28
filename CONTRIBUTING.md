# Contributing to GENOS

GENOS is an early-stage open source project built in public, one task per day.
We are in pre-revenue phase — contributions are currently voluntary.

---

## Current phase — pre-revenue

The project has no funding or treasury yet. If you contribute now, you are
doing so as an open source collaborator, not as a paid contractor.

What you get today:
- Your name in the contributors list
- Early access to the project roadmap
- Direct influence over architecture decisions
- First position when the treasury activates

---

## Planned reward model — activates with first revenue

Once the project reaches paying users, a USDC treasury smart contract
on Base (Ethereum L2) will activate automatic distribution:

| Contribution | Planned reward | Vesting |
|---|---|---|
| Merged feature PR | $200–1,000 USDC | 90 days |
| Critical bug fix | $500–2,000 USDC | Immediate |
| Provider manifest | $50 USDC + 5% revenue share | Forever |
| New approved agent | $300–800 USDC + 3% of Pro subs | 90 days |

Contributions made before treasury activation will be logged on-chain
and retroactively rewarded when revenue begins.

---

## How to contribute

1. Fork the repository
2. Create a branch: `git checkout -b feat/your-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Open a PR with a clear description of what and why
6. Add your Base wallet address to the PR description for future USDC payment

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
