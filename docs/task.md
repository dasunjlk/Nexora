# Nexora — task board

Status of every task in the build-out of Nexora. Updated at the end of each
task. **One short-lived branch per sub-task**, small granular commits inside
it, guideline written before implementation (`docs/guidelines/NN-*.md`), and
**CI green on the branch before it merges** — a failure is fixed on the same
branch and re-pushed, never merged red.

**Long-lived branches:** `main` (stable) · `dev` (integration) · `testing`
(QA) · `production` (release mirror). Everything else is short-lived
(`feature/NNx-*`, `fix/*`, `docs/*`). See [branching.md](branching.md) and
[ci.md](ci.md).

## Task 01 — project docs & scaffolding ✅

| Sub-task | Branch | Status |
|---|---|---|
| Docs, README, branch model, meeting-notes spec, guidelines | `feature/01-project-docs` | ✅ merged |

## Task 02 — port the codebase + mechanical rebrand ✅

| Sub-task | Branch | Status |
|---|---|---|
| 02.0 | Task guideline | ✅ merged |
| 02a | Port the reference tree as-is (5 granular commits) | `feature/02a-port-codebase` ✅ |
| 02b | Rename tree: crates, src dirs, res, installer, brand files | `feature/02b-rename-tree` ✅ |
| 02c | Rebrand Rust names: `poni*` → `nexora*`, `studio-*` crates | `feature/02c-rebrand-rust-names` ✅ |
| 02d | Rebrand Oyenza names, domains, services (`oyis_` → `nxis_`) | `feature/02d-rebrand-oyenza` ✅ |
| 02e | Rebrand the mascot (`oyen*` → `nova*`) + drop legacy names | `feature/02e-rebrand-mascot` ✅ |
| 02f | Version 0.1.0 + installer/MSIX identity (new GUIDs) | `feature/02f-version-identity` ✅ |
| 02g | `Cargo.lock`, contract vectors, add-on pins, update sample; workspace tests green (1156 passed) | `feature/02g-lockfile-checks` ✅ |
| fix | exec bits for the shell scripts, weight-budget recalibration, install-test gate | `fix/02a-exec-bits` ⏳ CI |

## Task 03 — Nexora brand assets ✅

| Sub-task | Branch | Status |
|---|---|---|
| 03a | Logo + app icon + installer art (`tools/make_brand.py`, violet tile) | `feature/03a-logo-icon` ✅ |
| 03b | Phone-page icons + Nova illustrations (`server/make_icons.py`, `make_phone_art.py`) | `feature/03b-phone-art` ✅ |
| 03c | The app's mascot recoloured (FUR `#7C5CFF`, FOLD `#5B3FD6`); contrast fix folded into `fix/02a-exec-bits` | `feature/03c-mascot-recolor` ✅ |

## Task 04 — outer rebrand

| Sub-task | Branch | Status |
|---|---|---|
| 04a | Workflows: ffmpeg exec bit, weight budget, install-test gate, add-on pre-releases published (`addon-sholat-3`, `addon-ja-1`, `addon-downloader-1`), Pages enabled | `fix/02a-exec-bits` ⏳ CI |
| 04b | Website: fresh 192-DPI screenshots (Classic/Nova/Pill/Diorama), Nexora's own pose art (`tools/make_brand_art.py`), 0.1.0 news band, violet tokens | `feature/04b-site` ⏳ CI |
| 04c | Server: phone-page accents violet; tests 59/59 | `feature/04c-server` ⏳ CI |
| 04d | Root docs: brand-kit README rewritten, upstream plush art removed, PRIVACY coherent | `feature/04d-root-docs` ⏳ CI |

## Task 05 — build & test ✅

| Sub-task | Branch | Status |
|---|---|---|
| Release build (3m44s), self-test 144/144 states (classic, Nova, Pill, Diorama at 192/200 DPI), report clean; the stocks frame-time check is machine-load-sensitive and passes when quiet (CI's software renderer reports, not fails) | `feature/05-build-test` ✅ |

## Task 06 — merge ladder & CI ✅

| Sub-task | Branch | Status |
|---|---|---|
| 06a | `fix/02a-exec-bits` (with 03c folded in) — CI green → merged to `dev` | ✅ |
| 06b | `feature/04b-site`, `feature/04c-server`, `feature/04d-root-docs`, `docs/task-board-03` — each CI green → merged to `dev`; dev re-verified green | ✅ |
| 06c | `dev` → `testing` — CI green | ✅ |
| 06d | `testing` → `main` — CI + store + mac green (the Glass smoke flake and the Mac entitlements fix, `hotfix/mac-entitlements`, verified on main) | ✅ |
| 06e | `main` → `production` — CI green | ✅ |
| 06f | Tag **v0.1.0** on `main` — release run green: `Nexora-Setup-0.1.0.exe`, `Nexora-Setup.exe`, the portable zip, `Nexora-0.1.0-mac.dmg` + `Nexora-Mac.dmg` published; `latest.json` signed (UPDATE_FEED_KEY) and the website live at dasunjlk.github.io/Nexora | ✅ |
| fix | the update feed's trust list + secret (`fix/update-feed-key`) | ✅ merged to `main` and `dev` |

## CI notes (learned the hard way)

- Commits from Windows drop the executable bit — every `.sh` needed
  `git update-index --chmod=+x`.
- The self-test's Nova-theme scenes check a ≥140 channel contrast against
  the theme's ground: one accent colour must carry both porcelain and black.
- The Diorama picture checks need **192 DPI** (exact 2× pixel doubling);
  200 DPI misaligns the pixel-perfect picture.
- CI's software renderer reports slow frames instead of failing; a real GPU
  enforces the 16 ms budget (stocks-edit sits at ~15–16 ms on an iGPU at
  125 % scale, load-sensitive).
- The app downloads its add-on files from this repo's pre-releases —
  `addon-sholat-3`, `addon-ja-1`, `addon-downloader-1` are published here,
  byte-checked against the pins in `nexora_core::addons`.
- `services::applemusic_ui::tests::presses_apple_musics_own_controls` is
  timing-sensitive on runners; re-run failed jobs when it flakes.

## Notes

- The reference implementation lives in a gitignored local folder
  (`oyenza-studio-main/`, upstream MIT, credited in
  [rebranding.md](rebranding.md)); it is never committed.
- The Nexora-branded service endpoints (api/drop/inference/trade) describe
  the client contracts; no instance is deployed yet — those features
  degrade gracefully until a service is pointed at.
