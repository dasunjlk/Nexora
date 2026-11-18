# Nexora — branching strategy

## Branches

| Branch | Purpose | Receives | CI |
|---|---|---|---|
| `main` | Stable. Default branch. Website publishes from it; release tags are cut here. | merges from `dev` (release trains) and hotfix branches | full CI on every push; release jobs on tags |
| `dev` | Integration. Everything lands here first. | merges from `feature/*` (fast-forward or merge commit), hotfixes | full CI on every push |
| `testing` | QA build. A snapshot of `dev` under test before promoting to `main`. | merges from `dev` | full CI + smoke on every push |
| `production` | Release mirror. Points at commits that are (or are about to be) released. | merges from `main` only | full CI on every push |
| `feature/NN-slug` | One task per branch, named after the task number in `docs/task.md`. | created from `dev`, merged into `dev` | CI on push |
| `hotfix/*` | Urgent fixes for a released `main`. | created from `main`, merged into `main` **and** `dev` | CI on push |

## Rules

1. **Never commit directly to `main`, `testing` or `production`.** Work
   happens on `feature/*` (or `hotfix/*`) branches.
2. **A merge follows a green CI run.** Push the feature branch, wait for the
   pipeline, fix until green, then merge.
3. **Merge direction:** `feature/*` → `dev` → `testing` → `main` →
   `production`. Nothing skips a level except hotfixes (which land on `main`
   and are merged back to `dev` immediately).
4. **Release tags** (`vX.Y.Z`) are annotated tags on `main`; the CI release
   jobs build and publish from the tag. After a release, `main` is merged
   into `production`.
5. **Version bumps** happen on `dev` (workspace version + installer files +
   `Cargo.lock`, kept in sync).
6. **Task docs** (`docs/task.md`) are updated in the same merge that completes
   a task.

## Daily flow used to build Nexora

```
main ──────────────┬──────────────► (tag vX when releasing) ──► production
                   │ merge dev                                (merge from main)
dev ──┬──┬──┬───┬────┴────────────────►
      │  │  │   │
      │  │  │   └─ feature/06-merge-ladder
      │  │  └─ feature/04a-workflows … 04d-root-docs
      │  └─ feature/03a-logo-icon, 03b-installer-phone-art
      └─ feature/02a-port-codebase, 02b-rename-tree, 02c…02g rebrand passes,
         05-build-test …
testing ◄── merge from dev (QA snapshot)
```

- **One short-lived branch per sub-task** (see `docs/task.md`), created from
  the current tip of `dev`, merged back with `--no-ff` as soon as it is
  green. Each branch carries small, granular commits (one logical step per
  commit) so history reads like a changelog.
- `testing` and `main`/`production` receive the result at the milestones
  defined in `docs/task.md`.
