# Implementation guideline — Task 08: everything is free (no Pro)

**Branch:** `fix/no-pro` → merged into `dev`
**Date:** 2026-11-19
**Status:** implemented

## Problem

The port kept the upstream's **Pro** tier: add-ons flagged `pro`, rows
locked behind "Pro" with a "+" that opened a sales card, the IELTS and
Downloads tabs gated on the account's Pro state, the Notes page showing a
Pro card instead of its record button, and the website selling a pricing
section. In Nexora **everything is free** — the gates, tags and upsells
come out, and the app stops asking the account server about Pro at all.

## Change, commit by commit

1. **`nexora-core::addons`** — drop the `pro: bool` field from `Addon` and
   every initializer; its one read site is the page below.
2. **`pages/addons.rs`** — remove `needs_pro`/`locked` and the Pro card
   ("See Studio Pro" + the early-access line): every add-on row is a switch
   with its size (or "Built in"); no "Pro" tag. Tests rewritten for the
   all-free behaviour.
3. **Account plumbing** — `state.rs` (`nexora.pro`, `studio_pro()`, the
   IELTS/Downloads gates), `events.rs` (`NexoraEvent::Pro`), `commands.rs`
   (the pro ask), `app.rs` (the event handling, the Notes page's Pro-card
   path), `services/oyenza.rs` (the server ask), the Mac's
   `app_downloads.rs`: all of it out. The AI tab and the AI-usage service
   are ungated by account, exactly like every other feature.
4. **Self-test scenes** — the `NotesPro` scene and the per-scene Pro
   seeding go; the scenes' screenshots stay.
5. **Website** — the pricing section, the "Pro" tags in the sidebar and the
   Pro lines in the copy go; the add-ons section says every add-on is
   included, free.
6. **Docs** — `PRIVACY.md` (the Pro-ask sentence; the AI-usage heading's
   "only with Studio Pro"), `ARCHITECTURE.md`, `docs/features/meeting-notes.md`
   (the wire keeps a `pro_required` error mapping — the server may still
   answer it — but the client never gates), `docs/task.md`, `docs/design.md`.

## Kept

- The wire's `pro_required` error mapping in `nexora_core::notes`/`ielts`
  (it describes a server answer, not a client gate).
- The tab structure and every feature: Meeting notes, IELTS, Downloader,
  AI usage, Recorder — all switchable add-ons like Health or System, free.

## Verification

- `cargo test --workspace` green (tests that asserted the Pro behaviour are
  rewritten or removed with the feature).
- Self-test green (scene count changes: `NotesPro` is gone).
- `grep -i "pro"` over the app's user-visible strings returns only
  unrelated words (process, provide, …).
