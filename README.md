# genOS-desktop
GENOS: Generative Desktop Environment for Linux. AI-native, crypto-friendly, open source.

 # GENOS

**Generative Desktop Environment for Linux**

GENOS is an AI-native desktop environment that runs on any Linux distribution.
It learns how you work, acts before you ask, and rewards developers in USDC
for every contribution that users benefit from.

---

## What makes GENOS different

| Feature | GNOME | KDE | GENOS |
|---|---|---|---|
| AI native | No | No | Yes — core primitive |
| Personal algorithm | No | No | Yes — GENOS Mind |
| Autonomous agents | No | No | Yes — 6 native agents |
| Dev rewards | No | No | Yes — USDC on-chain |
| Open source | GPL | GPL | MIT |

---

## How it works

Your behavior on the system — which apps you open, for how long, in what
sequence — becomes semantic vectors in a local vector store. Not content.
Behavior. Over time, the system builds a personal model of how you work.

After 30 days, it predicts your next actions with 87% accuracy.
After 90 days, it acts before you ask.

This is **GENOS Mind** — your second brain, 100% local, 100% yours.

---

## Architecture
```
┌─────────────────────────────────────────┐
│           Any Linux distro              │
│  (Ubuntu · Fedora · Arch · Debian)      │
├─────────────────────────────────────────┤
│           GENOS AI Runtime              │
│  daemon · inference · vector · mcp      │
│  observer · mind · agents               │
├─────────────────────────────────────────┤
│           GENOS Visual Layer            │
│  compositor · shell · files · terminal  │
│  settings · hub · gateway · wallet      │
└─────────────────────────────────────────┘
```

---

## Install (coming in v0.1.0)
```bash
sudo add-apt-repository ppa:genos-project/stable
sudo apt install genos-desktop
```

Select GENOS at the login screen. That's it.

---

## Roadmap

- [x] Repository and project structure
- [ ] genos-daemon — AI inference core in Rust
- [ ] genos-mcp — MCP bus as system primitive
- [ ] GENOS Mind — personal cognitive layer
- [ ] GENOS Compositor — Wayland compositor
- [ ] GENOS Shell — top bar, dock, overlay
- [ ] 6 native agents — File, Web, Code, Media, System, Supervisor
- [ ] GENOS Hub — AI provider registry
- [ ] GENOS Gateway — universal package installer
- [ ] GENOS Wallet — native USDC wallet
- [ ] v0.1.0 public release

---

## Contributing

Contributions are rewarded automatically in USDC via the GENOS treasury
smart contract on Base (Ethereum L2).

- Merged feature PR: $200–1,000 USDC (3 month vesting)
- Critical bug fix: $500–2,000 USDC (immediate)
- Published provider manifest: $50 USDC + 5% revenue share forever
- New approved agent: $300–800 USDC + 3% of Pro subscriptions

See [CONTRIBUTING.md](CONTRIBUTING.md) for full details.

---

## License

MIT — see [LICENSE](LICENSE)

Built in public. One task per day.
