# Nexora — CI pipelines

Workflows live in `.github/workflows/` and run on GitHub Actions for every
push (with `concurrency` cancelling superseded runs of the same branch).

| Workflow | File | Runs on | What it does |
|---|---|---|---|
| **ci** | `ci.yml` | every push, tags, manual | The main pipeline: core logic tests (Linux), build `nexora.exe` (Windows), unit tests (Windows), self-test screenshots, Apple Music controls test, live-market test, Bluetooth filter test, downloader end-to-end test, phone-drop server tests, installer build (Inno Setup + NSIS + MSIX), notices/tokens checks; on a tag: release publishing + website hand-off |
| **mac** | `mac.yml` | pushes to `main`/`dev`, `mac-*` branches, tags | Builds the macOS app (Apple Silicon + Intel), renders every island state, smoke-tests the real window, packages the DMG (signs/notarizes when release secrets exist) |
| **website** | `pages.yml` | pushes to `main` touching `site/**` or `docs/brand/**` | Builds the landing page into `_site/` and deploys GitHub Pages; carries the signed `latest.json` update feed |
| **store** | `store.yml` | manual + `main` | Builds the Microsoft Store (MSIX) package |
| **metrics** | `metrics.yml` | hourly on `main` | Records release download counters to the `metrics` branch |
| **ja-dict** | `ja-dict.yml` | manual + changes to dictionary sources | Rebuilds the Japanese readings dictionary and verifies it |
| **ja-compare** | `ja-compare.yml` | manual | Cross-checks the Japanese readings dictionary against upstream sources |
| **ytdlp-pin** | `ytdlp-pin.yml` | weekly + manual | Pins the yt-dlp/QuickJS versions used by the Downloader add-on |

## Branch behaviour

- `ci` runs on **every push to every branch** (main, dev, testing,
  production, feature/*). The merge rule in [branching.md](branching.md) is:
  no merge before its branch's run is green.
- `mac` runs on `main`/`dev` (and tags); it is skipped on small feature
  branches to keep the queue short.
- The **website** deploys only from `main` (GitHub Pages source).

## Release path

1. Merge `dev` → `main`, bump version everywhere it lives (workspace
   `Cargo.toml` ×4 entries in `Cargo.lock`, `installer/nexora.nsi`,
   `installer/nexora.iss`, site fallback version).
2. Push annotated tag `vX.Y.Z` on `main`.
3. `ci` builds, tests, publishes the GitHub release with
   `Nexora-Setup.exe`, the portable zip and the MSIX.
4. `mac` adds the DMG to the release.
5. The website job refreshes the landing page and `latest.json`.
6. Merge `main` → `production`.

## Waiting for CI (project rule)

After every push: poll the run for the pushed branch, fix any failure on the
same branch, push again, and only merge when the run is green. The merge
commit is pushed and its own run verified before the next step.
