# Nexora — product brief

**Nexora** is a "living notch": a native Dynamic Island for Windows 11 (and
macOS 14+). A small black island hangs from the top-centre of the screen,
shows live activities (media, timers, pop-ups) while collapsed, and expands
on hover into quick pages. It is written in Rust, draws with Direct2D/Direct3D
on Windows (AppKit/Core Graphics on macOS), has no web view, no runtime, and
is engineered to cost almost nothing while it sits on screen all day.

## Important details

| | |
|---|---|
| **Product name** | Nexora |
| **Executable** | `nexora.exe` (Windows), `Nexora.app` (macOS) |
| **Version** | 0.1.0 (workspace version, all crates) |
| **License** | Apache-2.0 (this repo); ported from an MIT-licensed upstream, credited in `THIRD-PARTY-NOTICES.txt` |
| **Repository** | `github.com/dasunjlk/Nexora` |
| **Website** | GitHub Pages: `dasunjlk.github.io/Nexora/` (published from `site/` + `docs/brand/` on `main`) |
| **Platform** | Windows 10/11 (primary), macOS 14+ (notch-wrapping variant) |
| **Language** | Rust (workspace: `nexora`, `nexora-core`, `nexora-usage`, `nexora-diorama`) + Node server (`server/`, no dependencies) |
| **Data on disk** | `%APPDATA%\Nexora\` (settings, data), `%LOCALAPPDATA%\Nexora\` (cache, log, sealed keys), `Documents\Nexora\Notes\` (meeting notes) |
| **Accounts** | Optional Nexora account (device-code sign-in); no account needed for core use |
| **Telemetry** | None. No ads, no analytics. |

## Feature map

- **Music, word by word** — controls for whatever plays (Spotify, Apple
  Music, browsers, any Windows media source); synced lyrics from LRCLIB or
  Apple Music (own token); pronunciation line under Japanese/Korean/Chinese.
- **Live activities at rest** — album art and sung line, focus timer, volume,
  charging, Bluetooth headphones, steps, prayer countdown.
- **Controls** — a main tab with the machine's quick controls on one page:
  Wi-Fi, Bluetooth, mobile hotspot, the power mode (Best power efficiency /
  Balanced / Best performance), Project (PC screen only / Duplicate /
  Extend / Second screen only) and airplane mode, each switchable from the
  island.
- **Pages** — Music, Today (calendar/agenda via optional account), Shelf,
  Tasks, Focus (Pomodoro), AI (AI coding-plan usage), Nexora (account tab);
  add-on pages: Health, System, Stocks, Caffeine, IELTS, **Notes (Meeting
  notes)**, Script, Recorder face, Downloads, Prayer times, Notifications.
- **Phone drop** — QR-paired phone-to-PC file transfer over the LAN (web page
  served by the app) or any network (WebRTC through `drop.nexora.app`, code in
  `server/`).
- **Show over games** — FPS/GPU/CPU readout from Windows' Present log.
- **Themes** — Classic (black), Nova (mascot theme), Glass, Pill, Diorama.
- **Add-ons** — download on first use, SHA-256-checked, deletable; Japanese
  readings, Prayer times, Downloader; the rest built in.
- **Meeting notes** — one click and the app records microphone + PC sound,
  the Nexora server writes structured notes, saved as Markdown in
  `Documents\Nexora\Notes`. Full spec: [features/meeting-notes.md](features/meeting-notes.md).
- **Updates** — signed update feed on the website; in-app updater on Windows.

## Design principles (from the upstream, kept)

1. **Keep it light.** Nothing runs at rest unless the person switched it on
   or its page is open; anything repeating rests while the PC is locked, the
   display is off or battery saver is on; features sleep until used.
2. **No OS calls in the logic crates.** `nexora-core`, `nexora-usage` and
   `nexora-diorama` are pure and unit-tested on any OS.
3. **Big data is an add-on**, never part of `nexora.exe` (downloaded,
   SHA-256-checked).
4. **Frames only while something moves.** A vblank-locked frame clock; parked
   otherwise.
5. **Everything the app knows, the person can read** in `PRIVACY.md`.
