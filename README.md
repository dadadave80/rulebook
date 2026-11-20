# Rulebook

**Nigeria's new CBN virtual-asset rules, as Soroban smart contracts.**

Rulebook is an open-source toolkit that turns the compliance controls the Central Bank of Nigeria now requires into onchain checks. Stellar-based fintechs, anchors and stablecoin issuers can plug them in instead of building them from scratch.

> Built for the **Stellar Buildathon 2026** (Nigeria & Ghana). **Status: pre-build.** The build week runs Nov 25–30, 2026. Nothing here is production-ready yet.

## The problem

On 5 November 2026, the CBN's *Regulations for the Operations of Non-Security Virtual Assets in Nigeria* came into force. Among other things:

- Stablecoin issuers must build freeze and deny-list controls into their tokens and report each use to the CBN within 24 hours.
- P2P platforms must confirm that the name and BVN on the paying bank account match the verified customer before releasing tokens.
- Stablecoins must be backed 1:1, with at least a 2% buffer.
- Existing operators must register with the CBN by 5 December 2026 and file a complete licence application by 4 May 2027.

Stellar-based fintechs serving Nigeria now need controls like these, and today each team builds them alone. Rulebook gives them a shared, open-source implementation.

## Modules

| Module | What it enforces | Status |
| --- | --- | --- |
| `case_freeze` | Freezes, unfreezes or claws back a wallet only against an open case (court order, regulatory directive or NFIU instruction). Every action emits an event for the 24-hour CBN notice. | Build week |
| `p2p_escrow` | Releases escrowed tokens only after a KYC provider's signed BVN/name-match attestation is verified onchain. Refunds the seller after a timeout. | Build week |
| `reserve_guard` | Blocks minting unless the latest custodian attestation shows reserves of at least 102% of supply. | Stretch goal |
| `watcher` | Reads contract events from Stellar RPC and drafts the 24-hour CBN notice. | Build week |
| `dashboard` | Case log, P2P releases, reserve ratio and report exports for compliance officers. | Build week |

## How it works

```
KYC provider (mocked) ── signs BVN/name-match attestation ──▶ P2P app
P2P app ── release(trade, attestation) ──▶ p2p_escrow ── verifies attester signature ──▶ token transfer

Compliance officer ── open_case, freeze(account) ──▶ case_freeze ──▶ token admin (authorization / clawback)
                                                         └── events ──▶ watcher ──▶ draft CBN notice

Custodian (mocked) ── signs reserve attestation ──▶ reserve_guard ── mints only if reserves ≥ 1.02 × supply
```

## Why Stellar

- **Rules live in the asset.** Soroban contracts enforce controls at the token level, so an app backend can't skip them.
- **Issuer controls are built in.** The Stellar Asset Contract gives its admin authorization and clawback, and OpenZeppelin's Stellar token library adds pluggable compliance hooks.
- **Every action is auditable.** Contract events give auditors and regulators a verifiable record of each freeze, release and mint.
- **Checks are cheap.** Sub-cent fees and fast settlement make a compliance check on every trade affordable.

## Tech stack

- Contracts: Rust, `soroban-sdk`, OpenZeppelin Stellar contracts
- Watcher: TypeScript, `@stellar/stellar-sdk`
- Dashboard: Next.js
- Network: Stellar testnet only

## Roadmap

- **Build week (Nov 25–30):** `case_freeze` and `p2p_escrow` on testnet, the watcher, and a demo dashboard.
- **30 days after the buildathon:** pilot with two Stellar-based Nigerian startups, ship `reserve_guard`, and add docs, test coverage and a threat model.
- **Later:** Form M matching for stablecoin import payments, redemption operations tooling, and zero-knowledge attestations that let apps confirm a BVN match without ever seeing the BVN.

## Repository layout

```
contracts/
  case_freeze/
  p2p_escrow/
  reserve_guard/
packages/
  watcher/
apps/
  dashboard/
docs/
```

## Getting started

Setup instructions land during build week. You'll need Rust, the Stellar CLI and Node.js.

## Team

- **David Dada**: solo builder covering smart contracts, frontend, backend and design ([GitHub](https://github.com/dadadave80) · [X](https://x.com/dadadave80))

## Disclaimer

Rulebook provides technical controls that help teams implement regulatory requirements. It is not legal advice and does not make any operator compliant. Testnet only.

## License

Apache-2.0
