# Nexora — task board

Status of every task in the build-out of Nexora. Updated at the end of each
task. One feature branch per task, guideline written before implementation
(`docs/guidelines/NN-*.md`), CI green before a merge.

**Branches:** `main` (stable) · `dev` (integration) · `testing` (QA) ·
`production` (release mirror) · `feature/NN-*` per task. See
[branching.md](branching.md) and [ci.md](ci.md).

| #  | Task | Branch | Guideline | CI | Status |
|----|------|--------|-----------|----|--------|
| 01 | Project docs & repo scaffolding (`docs/`, README, branch model) | `feature/01-project-docs` | [01-project-docs.md](guidelines/01-project-docs.md) | n/a (no workflows yet) | ✅ done |
| 02 | Port the reference codebase + mechanical rebrand (`nexora` crates, `nexora.exe`, `NEXORA_*`, paths, domains) | `feature/02-port-rebrand` | [02-port-rebrand.md](guidelines/02-port-rebrand.md) | pending | ⏳ in progress |
| 03 | Nexora brand assets (logo SVG/PNG/ICO, installer art, phone-page icons) | `feature/03-brand-assets` | [03-brand-assets.md](guidelines/03-brand-assets.md) | pending | ⬜ pending |
| 04 | Site, server, installers, workflows, root docs rebrand (README/AGENTS/ARCHITECTURE/PRIVACY) | `feature/04-outer-rebrand` | [04-outer-rebrand.md](guidelines/04-outer-rebrand.md) | pending | ⬜ pending |
| 05 | Local build & test (workspace tests, release build, self-test smoke) | `feature/02-port-rebrand` (same branch) | [05-build-test.md](guidelines/05-build-test.md) | pending | ⬜ pending |
| 06 | Merge ladder with CI green: `feature/*` → `dev` → `testing` → `main` → `production` | — | [06-merge-ladder.md](guidelines/06-merge-ladder.md) | pending | ⬜ pending |
| 07 | Meeting notes feature documentation (complete implementation markdown) | `feature/01-project-docs` | [01-project-docs.md](guidelines/01-project-docs.md) | n/a | ✅ done ([features/meeting-notes.md](features/meeting-notes.md)) |

## Done criteria

- Every merge lands on a branch only after its CI run is green.
- After each merge, `main` stays deployable; `production` only ever receives
  merges from `main`.
- This table is the single source of truth for what is done and where.

## Notes

- The reference implementation lives in a gitignored local folder
  (`oyenza-studio-main/`, upstream MIT, credited in
  [rebranding.md](rebranding.md)); it is never committed.
- CI runs on every push once Task 02 lands (workflows arrive with the port).
