# Implementation guideline — Task 09: the Controls tab

**Branch:** `feature/07-controls-tab` → merged into `dev`
**Date:** 2026-11-19
**Status:** implemented

## Goal

A new core tab, **Controls**, with the machine's quick controls on one page,
each controllable from the island: **Wi-Fi, Bluetooth, Mobile hotspot,
Energy saver (power mode), Project (display mode) and Airplane mode** — the
things Windows keeps in its own quick settings, one hover away.

## Design

- **The tab** — `PageKind::Controls`, a hideable core tab (Settings › Tabs),
  placed after Focus in the row. Its page: a 2×3 grid of tiles (Wi-Fi,
  Bluetooth, Hotspot, Energy saver, Project, Airplane mode) drawn with the
  page widgets; tapping a tile toggles it; Energy saver is a three-way
  picker (Best power efficiency / Balanced / Best performance) and Project a
  four-pill row (PC screen only / Duplicate / Extend / Second screen only).
- **`nexora_core::controls` (pure)** — the page's state machine
  (`ControlsState`: radio states, hotspot + client count, power mode,
  project topology, display count; `None` = not known yet), the airplane
  logic (airplane ON remembers each radio's previous state and restores it
  on OFF; radios turned off by hand while in airplane stay off when it
  lifts), and the tile order. No OS calls; unit-tested everywhere.
- **`services/controls.rs` (Windows)** — one thread, started the first time
  the tab opens and dropped with the page (nothing runs at rest, per
  AGENTS.md): WinRT `Windows.Devices.Radios` (enumerate + `StateChanged`
  events for Wi-Fi/Bluetooth, `SetStateAsync` to toggle),
  `NetworkOperatorTetheringManager` (hotspot state, clients,
  `StartTetheringAsync`/`StopTetheringAsync`), `PowerSetActiveOverlayScheme`
  (the three power modes; the active one read from the Power key),
  `SetDisplayConfig` with a supplied topology for Project (PC screen only /
  Duplicate / Extend / Second screen only). Reports
  `AppEvent::Controls(ControlsState)`; takes `Command::Controls(ControlsCmd)`.
- **Airplane mode** — a client-side concept: ON turns every radio off
  (remembering what was on), OFF restores exactly those. Windows' own
  airplane setting isn't publicly scriptable; this matches what it does.
- **Testing, the hard rule**: no automated test ever turns a radio off. The
  self-test runs the service in a canned mode (fake states, toggles that
  only flip the local model) so screenshots work without touching the
  machine's radios. Unit tests cover the pure logic. Toggles happen only
  when the person clicks.
- **Costs** — nothing runs unless the tab is open (the service exists only
  while it is); radio `StateChanged` events push updates while it is open;
  no polling.

## Commits

1. `docs: guideline` (this file) + product/design/task rows.
2. `nexora_core::controls` + tests (pure).
3. Plumbing: `PageKind::Controls`, `ControlsCmd`/`AppEvent::Controls`,
   Settings' hideable-tab support, icons (Wi-Fi, Hotspot, Airplane, Project,
   Energy saver).
4. `services/controls.rs` + Cargo features (`Devices_Radios`,
   `Networking_NetworkOperators`).
5. `pages/controls.rs` (the grid) + the tab's row/order integration.
6. Self-test scene `page-controls` + README/ARCHITECTURE/task.md updates.

## Acceptance

- `cargo test --workspace` green (pure-logic tests for the state machine).
- Self-test green with a `page-controls` screenshot showing the six tiles.
- The Windows build compiles with the two new WinRT feature sets.
- No test or self-test path toggles a radio.
