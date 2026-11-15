# Nexora — rebranding policy & mapping

Nexora is a faithful port of an upstream open-source "living notch" app
(MIT-licensed; credited in `THIRD-PARTY-NOTICES.txt`). Everything
user-visible was rebranded to Nexora. This page is the mapping and the rules.

## Name mapping

| Upstream | Nexora | Where |
|---|---|---|
| Oyenza Studio (product) | **Nexora** | all user-visible strings, docs, installers, site |
| Oyen Poni / Poni (old product names) | removed | history notes |
| `poni` / `Poni` (internal codename) | `nexora` / `Nexora` | crates (`nexora`, `nexora-core`, `nexora-usage`, `nexora-diorama`), `nexora.exe`, `NEXORA_*` env vars, `%APPDATA%\Nexora`, `%LOCALAPPDATA%\Nexora`, `Local\Nexora.Island`, Mac bundle id `app.nexora.nexora`, uninstall key, Run value, threads (`nexora-notes`), self-test vars |
| Oyen (mascot cat) | **Nova** | `nexora_core::nova` (`src/nova/`), `src/nova.rs`, the "Nova" theme, `Nova's notes`, docs |
| `oyenza.app` (account) | `nexora.app` | sign-in, money/agenda features (service placeholder) |
| `api.oyenza.app` | `api.nexora.app` | account API, meeting notes API (`/island/notes`) |
| `drop.oyenza.com` | `drop.nexora.app` | paired phone drop relay (`server/`) |
| `inference.oyenza.com` | `inference.nexora.app` | AI tab's inference balance |
| `oyenza.trade` | `nexora.trade` | Stocks page prices |
| `studio.oyenza.com` | `dasunjlk.github.io/Nexora/` | website, update feed |
| `lucacadalora/oyenza-studio` | `dasunjlk/Nexora` | repo URLs, release/download links, badges |
| `oyen-drop-server` (npm name) | `nexora-drop-server` | `server/package.json` |
| `catat-server` (notes backend codename) | `nexora notes server` | docs only |
| Jatevo Labs (upstream copyright) | kept as upstream attribution only | `THIRD-PARTY-NOTICES.txt` |
| Oyen drop-server paths/images (`oyen-*.webp`) | `nova-*.webp` | `server/public/` |

## Rename policy

**Renamed** (mechanically, case-preserving): `oyenza→nexora`, `Oyenza→Nexora`,
`OYENZA→NEXORA`, `poni→nexora`, `Poni→Nexora`, `PONI→NEXORA`, `oyen→nova`,
`Oyen→Nova`, `OYEN→NOVA`, plus domains and repo slugs above.

**Never renamed** (would break third-party contracts, not branding):
- External API wire formats: LRCLIB, Apple Music, GitHub API, GeoNames, IDX
  market data field names.
- Upstream crate/protocol tokens inside `third_party/` (BSD-3 MediaRemote
  adapter).
- The Japanese dictionary format, add-on file names that are hashes/manifests.
- Licence texts (`OFL-Outfit.txt`, third-party notices content).

**Regenerated, not renamed:** the logo, app icon, installer art, phone-page
icons, mascot images — new Nexora art produced by the ported generators
(`tools/make_brand.py`, `server/make_icons.py`).

## Deliberate service notes

The Nexora-branded service endpoints (`api.nexora.app`, `drop.nexora.app`,
`inference.nexora.app`, `nexora.trade`) describe the contracts the client
implements; no public instance is deployed with this repository yet. Features
that need them (account sign-in, paired-phone relay, meeting notes writing,
inference balance, live stocks) degrade gracefully: the app shows the feature
as unavailable/offline until a service is pointed at. The relay server code
ships in `server/` and can be run anywhere (see `server/README.md`).

## Version

The workspace version starts at **0.1.0** for Nexora (upstream continued at
0.2.30). `Cargo.lock`, both installer scripts and the site's fallback version
must stay in sync (CI checks).
