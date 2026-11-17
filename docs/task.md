# Nexora — task board

Status of every task in the build-out of Nexora. Updated at the end of each
task. **One short-lived branch per sub-task**, small granular commits inside
it, guideline written before implementation (`docs/guidelines/NN-*.md`), CI
green before a merge into `dev`.

**Long-lived branches:** `main` (stable) · `dev` (integration) · `testing`
(QA) · `production` (release mirror). Everything else is short-lived
(`feature/NNx-*`, `hotfix/*`). See [branching.md](branching.md) and
[ci.md](ci.md).

## Task 01 — project docs & scaffolding

| # | Sub-task | Branch | Status |
|---|----------|--------|--------|
| 01 | Docs, README, branch model, meeting-notes spec, guidelines | `feature/01-project-docs` | ✅ merged to `dev` |

## Task 02 — port the codebase + mechanical rebrand

| # | Sub-task | Branch | Status |
|---|----------|--------|--------|
| 02.0 | Task guideline | merged with 01 | ✅ done |
| 02a | Port the reference tree as-is (5 granular commits) | `feature/02a-port-codebase` | ⏳ in progress |
| 02b | Rename tree: crates, src dirs, res, installer, brand files | `feature/02b-rename-tree` | ⬜ pending |
| 02c | Rebrand Rust names: `nexora*`/`NEXORA*` → `nexora*`, `studio-*` crates | `feature/02c-rebrand-rust-names` | ⬜ pending |
| 02d | Rebrand mascot: `nova*` → `nova*`, `nxis_` → `nxis_` | `feature/02d-rebrand-mascot` | ⬜ pending |
| 02e | Rebrand domains, product name, repo refs, `nexora-notes-server` | `feature/02e-rebrand-domains` | ⬜ pending |
| 02f | Version 0.1.0 + installer/MSIX identity (new GUIDs) | `feature/02f-version-identity` | ⬜ pending |
| 02g | Regenerate `Cargo.lock`, notices/tokens checks | `feature/02g-lockfile-checks` | ⬜ pending |

## Task 03 — Nexora brand assets

| # | Sub-task | Branch | Status |
|---|----------|--------|--------|
| 03a | Logo + app icon (`tools/make_brand.py` port: SVG, PNG, ICO) | `feature/03a-logo-icon` | ⬜ pending |
| 03b | Installer art + phone-page icons | `feature/03b-installer-phone-art` | ⬜ pending |

## Task 04 — outer rebrand (site, server, workflows, root docs)

| # | Sub-task | Branch | Status |
|---|----------|--------|--------|
| 04a | CI workflows adapted to Nexora | `feature/04a-workflows` | ⬜ pending |
| 04b | Website (site/) rebrand | `feature/04b-site` | ⬜ pending |
| 04c | Server (server/) rebrand + tests green | `feature/04c-server` | ⬜ pending |
| 04d | Root docs: AGENTS/ARCHITECTURE/PRIVACY/NOTICES/CLAUDE rebrand | `feature/04d-root-docs` | ⬜ pending |

## Task 05 — build & test

| # | Sub-task | Branch | Status |
|---|----------|--------|--------|
| 05 | Workspace tests, release build, self-test smoke, fixes | `feature/05-build-test` | ⬜ pending |

## Task 06 — merge ladder & CI

| # | Sub-task | Branch | Status |
|---|----------|--------|--------|
| 06 | `dev` → `testing` → `main` → `production`, CI green each step | `feature/06-merge-ladder` | ⬜ pending |

## Done criteria

- Every merge into `dev` lands only after its branch's CI run is green (once
  CI exists; Task 02a brings the workflows, Task 04a adapts them).
- After each merge, `dev` builds; `main` stays deployable; `production` only
  ever receives merges from `main`.
- This table is updated in the branch that completes the work.

## Notes

- The reference implementation lives in a gitignored local folder
  (`nexora-main/`, upstream MIT, credited in
  [rebranding.md](rebranding.md)); it is never committed.
