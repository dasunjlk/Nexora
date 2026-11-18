# Implementation guideline — Task 03c: recolour the app's mascot and Nova theme

**Branch:** `feature/03c-mascot-recolor` → merged into `dev`
**Date:** 2026-11-18
**Status:** implemented

## Goal

The app's cat still wears the upstream mascot's orange. Nexora's Nova is
recoulored to the brand's violets so the character reads as Nexora's own,
matching the logo and the phone-page art.

## Change

| What | Was | Now |
|---|---|---|
| Fur (`nova::ORANGE` → `nova::FUR`) | `0xF0892E` | `0xB49BFF` |
| Ear fold (`nova::FOLD_ORANGE` → `nova::FOLD`) | `0xC9581A` | `0x8F73F2` |
| Nova theme ground (light) | `0xFFF1DE` warm cream | `0xF3F0FF` cool porcelain |
| Nova theme ink (light) | `0x2B1B10` warm brown | `0x271E4A` deep indigo |
| Nova theme ground/ink (dark variant) | warm equivalents | the same cool pair |
| `docs/brand/nova-mark.svg` fills | `#F0892E` / `#C9581A` | `#B49BFF` / `#8F73F2` |

- The identifiers rename (`ORANGE` → `FUR`, `FOLD_ORANGE` → `FOLD`) so the
  names no longer lie about their colour.
- The theme's functional colours (green/red/blue/cyan/purple/yellow/pink
  swaps) stay standard.
- `nova/svg.rs`'s tests assert the hexes; they move with the values. The
  Swift-port fixture check (`swift_check.rs`) is geometry-only (`ink:
  "currentColor"`), so the fixture is untouched.
- The theme constants live in `nexora-core/src/look.rs` (the `NOVA` look and
  its ink); any warm-cream hex in that block moves to the cool pair.

## Acceptance

- `cargo test --workspace` stays green (including `nova`'s own tests).
- No `0xF0892E` / `0xC9581A` / `#F0892E` remains outside `THIRD-PARTY-NOTICES`
  attribution and this guideline.
