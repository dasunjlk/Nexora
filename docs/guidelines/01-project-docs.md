# Implementation guideline — Task 01: Project docs & repo scaffolding

**Branch:** `feature/01-project-docs` → merged into `dev`
**Date:** 2026-11-16
**Status:** implemented

## Goal

Lay the documentation foundation of the Nexora repository before any code is
ported: the planning docs, the branching/CI rules, the rebranding policy and
the complete Meeting notes feature specification. Every later task starts by
writing its guideline here and ends by updating `docs/task.md`.

## Scope

1. `docs/task.md` — the task board: every task, its branch, its CI status.
2. `docs/product.md` — brief product description with the important details.
3. `docs/design.md` — design doc: brand (name, colours, mascot, logo), UX and
   system design summary.
4. `docs/branching.md` — the branch model and merge rules.
5. `docs/ci.md` — the CI pipelines and what each one does.
6. `docs/rebranding.md` — the old→new name mapping and the rename policy
   (what is renamed, what never is, why).
7. `docs/features/meeting-notes.md` — **complete** implementation markdown for
   the Meeting notes feature (contract, state machines, API, files, tests).
8. `docs/guidelines/` — one implementation guideline per task, written before
   the task is implemented.
9. `README.md` — a normal, concise project README (the previous 61 KB upstream
   README is intentionally not carried over; it can be rebuilt later).

## Non-goals

- No code is ported or renamed in this task (that is Task 02).
- No CI workflows are added yet (they arrive with the code in Task 02/04; CI
  begins to run on the first push that carries `.github/workflows`).

## Rules observed

- All documentation lives under `docs/`.
- Docs are written in Markdown, in plain factual language.
- Upstream project (the reference implementation) is credited; its name
  appears only in attribution/licensing contexts, never as product branding.
- Nothing outside `docs/` and `README.md` changes in this task.

## Acceptance

- All files above exist, are consistent with each other and with the branch
  model (`main`, `dev`, `testing`, `production`, `feature/*`).
- `docs/task.md` lists this task as done.
