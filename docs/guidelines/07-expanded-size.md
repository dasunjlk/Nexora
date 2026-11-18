# Implementation guideline — Task 07: smaller expanded island

**Branch:** `fix/expanded-size` → merged into `dev`
**Date:** 2026-11-19
**Status:** implemented

## Problem

The island opens at **620 DIP** wide by default; on the tester's screen the
expanded card feels too large (the reference screenshot shows the card a
step smaller). The width is already user-adjustable (Settings › Island ›
**Width when open**, 480–900 DIP, pages reflow at any width), but the
default should start smaller.

## Change

- `Settings::expanded_width` default: **620 → 540** (both the `Default`
  impl and the sanitize fallback), −13 %.
- The clamp (480–900) and the stepper stay: the person can widen it again.
- Pages already lay out against the body width, so nothing else moves; the
  self-test's `expanded-width` check reads the setting, so it follows.

## Verification

- `cargo test --workspace` green (the settings tests that hard-code 620 in
  unrelated scenarios are checked and, where they assert the default,
  updated).
- Self-test screenshots at 125 % DPI re-taken and eyeballed: the Music,
  System and Notes pages must not truncate at 540.

## Docs

- `docs/design.md` (the width line) and `docs/task.md` (task row).
