# Implementation guideline — Task 02: Port the codebase + mechanical rebrand

**Branch:** `feature/02-port-rebrand` → merged into `dev`
**Date:** 2026-11-16
**Status:** implemented

## Goal

Bring the reference implementation into the Nexora repository in full and
rebrand it mechanically, so the app builds and tests as **Nexora** with no
upstream branding left.

## Inputs

- The gitignored local reference tree `nexora-main/nexora-main/`
  (upstream MIT implementation, ~132k lines of Rust + Node server + website +
  installers + CI).
- `docs/rebranding.md` — the mapping and rename policy (authoritative).

## Steps

1. **Copy** the reference tree into the repo root, excluding its `README.md`
   and `LICENSE` (Nexora's own already exist: concise README; Apache-2.0
   license). Copy `.gitignore` entries and `.gitattributes` from the
   reference; keep the `nexora-main/` ignore line.
2. **File and directory renames** (git records them as renames):
   `crates/nexora → crates/nexora`, `crates/nexora-core → crates/nexora-core`,
   `crates/nexora-usage → crates/nexora-usage`,
   `crates/nexora-diorama → crates/nexora-diorama`,
   `src/oyen.rs → src/nova.rs`, `src/oyen/ → src/nova/`,
   `res/nexora.{ico,rc,manifest} → res/nexora.*`,
   `installer/nexora.{iss,nsi} → installer/nexora.*`,
   brand asset filenames (`oyen-mark* → nova-mark*`, `nexora-logo* →
   nexora-logo*`, mascot images `oyen-* → nova-*`).
3. **Token replacements** across every text file, longest-first so compound
   names resolve once:
   repo slug → `dasunjlk/Nexora`; `app.nexora.nexora` → `app.nexora.nexora`;
   `Nexora`/`nexora` → `Nexora`/`nexora`; service domains
   (`api.nexora.app → api.nexora.app`, `drop.nexora.app → drop.nexora.app`,
   `inference.nexora.app → inference.nexora.app`, `nexora.trade →
   nexora.trade`, `dasunjlk.github.io/Nexora → dasunjlk.github.io/Nexora`);
   `nxis_` → `nxis_`; `nexora-notes-server` → `nexora-notes-server`; then the
   case variants `Nexora`, `Nexora`, `nexora`, `Nexora`,
   `NEXORA`, `nexora`, `Oyen`, `OYEN`, `oyen`, `nexora-usage`/`nexora_usage`,
   `nexora-diorama`/`nexora_diorama`, `Nexora`, `NEXORA`, `nexora`.
4. **Version reset**: workspace version `0.2.30 → 0.1.0` in `Cargo.toml`,
   the CI env, both installer scripts and the website's fallback version;
   regenerate `Cargo.lock`.
5. **Installer identity**: new Inno Setup AppId and NSIS GUID for the new
   product (upstream GUIDs are theirs; Nexora installs as its own product).
6. **Manual review** of the files mechanical passes can't fully fix:
   upstream history sentences in `ARCHITECTURE.md`/`AGENTS.md`, workflow
   names/artifacts, the Mac `Info.plist`, the MSIX manifest identity, and
   the self-test's expected artifact names.
7. **Verify**: `cargo test -p nexora-core -p nexora-usage -p
   nexora-diorama`, then a release build of `nexora` and the scripted
   self-test (Task 05).

## Rules

- No functional code changes: behaviour stays identical; only names,
  strings, paths, domains and identities change.
- Third-party wire formats (LRCLIB, Apple Music, GitHub, GeoNames, IDX field
  names) and `third_party/` sources are not touched.
- The reference folder is never committed.

## Acceptance

- `grep -ri "nexora\|nexora\|oyen" --include="*"` over tracked text files
  returns only licence/attribution contexts.
- Workspace tests pass; `nexora.exe` builds.
