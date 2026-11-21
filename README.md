# Rulebook

**Virtual-asset compliance rules, as Soroban smart contracts.**

Rulebook is an open-source toolkit that turns the compliance controls virtual-asset regulators require into onchain checks. Each jurisdiction's requirements live in its own rule set, starting with Nigeria and Ghana. Stellar-based fintechs, anchors and stablecoin issuers can plug them in instead of building them from scratch.

> Built for the **Stellar Buildathon 2026** (Nigeria & Ghana). **Status: pre-build.** The build week runs Nov 25–30, 2026. Nothing here is production-ready yet.

## The problem

Virtual-asset rules keep coming back to the same few controls: freezing and deny-listing wallets on lawful orders and reporting each use quickly, checking identity before tokens are released, and backing stablecoins in full. What changes between jurisdictions is the detail: who can order a freeze, how fast the regulator must hear about it, which identifiers count and how much reserve is enough.

Stellar-based fintechs need controls like these in every market they serve, and today each team builds them alone. Rulebook gives them a shared, open-source implementation, with each jurisdiction's details kept in a rule set.

### Nigeria

On 5 November 2026, the CBN's [*Regulations for the Operations of Non-Security Virtual Assets in Nigeria*](https://www.cbn.gov.ng/Out/2026/CCD/REGULATIONS%20FOR%20THE%20OPERATIONS%20OF%20NON-SECURITY%20VIRTUAL%20ASSETS%20IN%20NIGERIA.pdf) came into force. Among other things:

- Stablecoin issuers must build freeze and deny-list controls into their tokens and report each use to the CBN within 24 hours.
- P2P platforms must confirm that the name and BVN on the paying bank account match the verified customer before releasing tokens.
- Stablecoins must be backed 1:1, with at least a 2% buffer.
- Existing operators must register with the CBN by 5 December 2026 and file a complete licence application by 4 May 2027.

### Ghana

Ghana's Parliament passed the *Virtual Asset Service Providers Act, 2025* (Act 1154) on 19 December 2025. The Bank of Ghana and the Securities and Exchange Commission (SEC) regulate under it, depending on the activity. So far:

- Exchanges, custodial wallet providers, brokers and similar intermediaries serving the public must be licensed or registered ([Bank of Ghana FAQ](https://www.bog.gov.gh/wp-content/uploads/2026/03/VASP-Act-FAQs.pdf)). Existing operators get transitional arrangements once the regime is operational ([joint notice, 20 February 2026](https://www.bog.gov.gh/wp-content/uploads/2026/02/PRESS-RELEASE-PUBLIC-NOTICE-ON-UNAUTHORISED-ADVERTISING-OF-VIRTUAL-ASSET-AND-STABLECOIN-PRODUCTS-200226.pdf)).
- Platforms must verify customer identity, and providers must comply with the Anti-Money Laundering Act, 2020 (Act 1044) ([Bank of Ghana FAQ](https://www.bog.gov.gh/wp-content/uploads/2026/03/VASP-Act-FAQs.pdf), [virtual assets page](https://www.bog.gov.gh/virtual-assets/)).
- The SEC's sandbox guidelines require virtual-asset applicants to have policies for KYC, transaction monitoring, suspicious transaction reporting and the travel rule, and require sandbox firms to report material incidents to the SEC within 24 hours ([SEC/GUI/001/03/2026](https://sec.gov.gh/wp-content/uploads/Final-Regulatory-Laws/Guidelines/Guidelines_Regulatory_Sandbox%20Licensing_2026.pdf)).
- The detailed operating rules are still being written. In August 2026 the Bank of Ghana's Governor said the regulators aim to fully operationalise the Act by 2027 ([remarks, 25 August 2026](https://www.bog.gov.gh/wp-content/uploads/2026/08/Opening-Remarks-by-Governor-Dr-Johnson-Pandit-Asiama-at-the-Inauguration-of-the-Virtual-Assets-Coordinating-Committee-.pdf)).

Ghana's rule set leaves its freeze-reporting, P2P identity and reserve values open until those rules are published.

## Modules

| Module | What it enforces | Status |
| --- | --- | --- |
| `case_freeze` | Freezes, unfreezes or claws back a wallet only against an open case (court order, regulatory directive or financial intelligence unit instruction). The jurisdiction's rule set decides which authority can order which action. Every action emits an event for the regulator notice the jurisdiction requires (Nigeria: the CBN within 24 hours). | Build week |
| `p2p_escrow` | Releases escrowed tokens only after a KYC provider's signed identity-match attestation (Nigeria: name and BVN on the paying bank account) is verified onchain. Refunds the seller after a timeout. | Build week |
| `reserve_guard` | Blocks minting unless the latest custodian attestation shows reserves at or above the jurisdiction's required ratio (Nigeria: 102% of supply). | Stretch goal |
| `watcher` | Reads contract events from Stellar RPC and drafts the regulator notice each jurisdiction requires. | Build week |
| `dashboard` | Case log, P2P releases, reserve ratio and report exports for compliance officers. | Build week |
| `rules` | One rule set per jurisdiction, starting with Nigeria and Ghana: who can order each action, notice deadlines, identity checks and reserve ratios, each linked to its source. | Build week |

Adding a jurisdiction for these controls is meant to take a new rule set, not new contracts.

## How it works

```
KYC provider (mocked) ── signs identity-match attestation ──▶ P2P app
P2P app ── release(trade, attestation) ──▶ p2p_escrow ── verifies attester signature ──▶ token transfer

Compliance officer ── open_case, freeze(account) ──▶ case_freeze ──▶ token admin (authorization / clawback)
                                                         └── events ──▶ watcher ──▶ draft regulator notice

Custodian (mocked) ── signs reserve attestation ──▶ reserve_guard ── mints only if reserves ≥ required ratio × supply

Rule set (NG, GH, …) ── authorities, deadlines, identity checks, ratios ──▶ contracts and watcher
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

- **Build week (Nov 25–30):** `case_freeze` and `p2p_escrow` on testnet, the watcher, and a demo dashboard, driven by the Nigeria and Ghana rule sets.
- **30 days after the buildathon:** pilot with two Stellar-based Nigerian startups, ship `reserve_guard`, and add docs, test coverage and a threat model.
- **Later:** rule sets for more jurisdictions, Form M matching for stablecoin import payments (Nigeria), redemption operations tooling, and zero-knowledge attestations that let apps confirm an identity match, such as a BVN, without ever seeing the identifier.

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
rules/
  ng.json
  gh.json
docs/
```

## Getting started

Setup instructions land during build week. You'll need Rust, the Stellar CLI and Node.js.

## Team

- **David Dada**: solo builder covering smart contracts, frontend, backend and design ([GitHub](https://github.com/dadadave80) · [X](https://x.com/dadadave80))

## Disclaimer

Rulebook provides technical controls that help teams implement regulatory requirements. It is not legal advice and does not make any operator compliant. Each rule set reflects its cited sources as of the date it was last checked. Testnet only.

## License

Apache-2.0
