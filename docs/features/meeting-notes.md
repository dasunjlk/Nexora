# Meeting notes — complete implementation

> Status in Nexora: **ported as-built**, rebranded (Nova → Nova, Nexora →
> Nexora, `api.nexora.app` → `api.nexora.app`). This document is the complete
> implementation reference: behaviour, state machines, wire contract, storage,
> failure handling and tests. It is written so the feature could be
> re-implemented from this file alone.

**Meeting notes** (the **Notes** add-on; built in, nothing to download;
Windows and macOS) adds a **Notes** tab: one click and Nexora listens to the
meeting, then writes it up. The app records the **default microphone and the
PC's own sound together** (so a call in Google Meet, Zoom, Teams or WhatsApp
is heard with no bot joining it, and an in-room meeting through the
microphone), and after **Stop** hands the audio to the Nexora server, which
returns structured notes. The notes are saved as a **Markdown file** in
`Documents\Nexora\Notes\` (macOS: `~/Documents/Nexora/Notes/`).

---

## 1. Product behaviour

### 1.1 The page (`pages/notes.rs`)

- A tab labelled **Notes**, present only while the **Meeting notes** add-on is
  on (`AppState::notes_on`); switching the add-on off removes the tab, stops
  any recording and clears leftover audio.
- Requires being signed in to the Nexora account (the server call carries the
  PC's account key). Signed out, the page offers **Sign in**.
- Controls:
  - **Record / Stop** button (a round record button under the mascot).
  - While listening: elapsed time (`0:07`, `12:34`, `1:02:03`), two level
    bars (microphone, PC sound), and Nova — the mascot's head moves with the
    voices.
  - Language picker for the notes: **Auto** (the meeting's own), **Indonesian**,
    **English** (`Lang`).
  - After Stop: a **Sending n/m parts** progress, then **Nova is writing**…
  - When done: the notes are shown with **Open notes**, **Copy** (the notes
    without the transcript, ready to paste) and **Show in folder**; the file
    name is displayed.
  - On failure: the problem in plain words (never a code), with **Try again**
    and **Throw away** where they make sense.
- The closed island shows Nova listening with the elapsed time
  (`Activity::Notes`) while a meeting is under way (ambient frames while it
  shows, nothing otherwise).
- A notice says when the notes are ready (you can close the island meanwhile).

### 1.2 What the user is told

- "Let the people in the meeting know you're taking notes." (page footer)
- Meetings up to **3 hours** are written up (the recording stops itself
  there).
- If something goes wrong, the recording stays on the PC for **Try again**
  until the notes are saved or it is thrown away.

### 1.3 Limits (server-side, mirrored client-side)

| Limit | Value |
|---|---|
| Longest meeting | `MAX_SECS = 3 h` (recording stops itself) |
| Audio part length | `PART_SECS = 300 s` (5 minutes; 9.6 MB WAV) |
| Poll while writing | `POLL_S = 4 s` |
| Give-up-on-writing | `WRITING_MAX_S = 20 min` |
| Audio format | 16 kHz, mono, 16-bit PCM (`RATE = 16_000`, shared with IELTS) |

---

## 2. Architecture

```
 pages/notes.rs (UI thread)            services/notes.rs                server
 ──────────────────────────           ───────────────────────          ────────
 Taker (nexora_core::notes)  ◄── AppEvent::Notes ──  Sender thread
   phase, mic/pc levels, last      ▲        nexora-nexora-send        api.nexora.app
        ▲                          │        POST /island/notes      /island/notes
        │ user actions             │        …/part/n  …/finish      /island/notes/:id
        ▼                          │        GET  /island/notes/:id
 Command::Notes ──► app.rs ──► Recording thread
                                "nexora-notes" (only while recording)
                                mic source ─┐
                                pc source  ─┴─► Mixer ─► Parts (WAV)
```

- **`nexora_core::notes`** (`crates/nexora-core/src/notes.rs`) — pure logic,
  no OS calls, unit-tested on any OS:
  - [`Mixer`] — the two sources onto one 16 kHz mono track, in step with the
    clock.
  - the calls' addresses and bodies and their answers' parsers (the contract),
  - [`Taker`] — the page's state machine, [`Phase`], [`Saved`], [`Reply`],
  - [`Problem`] — every failure in plain words (titles and lines),
  - [`markdown()`] — the notes as Markdown, [`file_name()`] — the file name.
- **`services/notes.rs`** (`crates/nexora/src/services/notes.rs`) — the
  Windows capture and the hand-over (the Mac app runs this same file, with
  its own two sources in `mac/services/capture.rs`):
  - [`Recording`] — the `nexora-notes` thread, only while recording,
  - [`Source`] — one WASAPI capture (mic) or loopback (render) endpoint,
  - [`Parts`] — the five-minute WAV part files,
  - [`Sender`] — the `nexora-notes-send` thread that hands the meeting over,
  - [`kept()`]/[`clear_audio()`] — the unfinished-meeting and cleanup logic.
- **The server** (not part of this repository; contract below) — receives the
  parts, transcribes, writes the notes, serves the status until fetched, and
  lets the client delete the meeting after it saved its copy.

## 3. The recording

### 3.1 Sources

- **Microphone**: the default capture device (`eCapture`, console role),
  WASAPI shared mode, opened by `services/mic.rs` (`open_endpoint`); each
  buffer is downmixed to 16 kHz mono (`nexora_core::ielts::Downmix`).
- **PC's own sound**: the default render device with WASAPI **loopback**
  (`eRender` + `AUDCLNT_LOOPBACK`); whatever the PC plays. Windows sends no
  loopback packets while nothing plays.
- One source missing is fine (no microphone; nothing playing). **Both**
  missing is `CaptureIssue::NoSound`. Windows' privacy switch blocking
  desktop apps from the microphone is `CaptureIssue::MicDenied` (checked with
  `mic::privacy_denies()`).
- **Device changes**: every 2 s (`DEVICE_CHECK`) each source compares
  Windows' current default device id with the one it opened; if it moved (a
  headset plugged in) or died, the default is opened again.
- Each source keeps a level meter: the loudest sample of the last ~100 ms,
  eased down (`level = max(level * 0.82, peak(pcm))`), shared to the UI thread
  as f32 bits in an `AtomicU32`.

### 3.2 The mixer (`nexora_core::notes::Mixer`)

- The two sources' queues are mixed **in step with the clock, not the
  packets**: `take(clock_samples)` emits exactly `clock − LAG_SAMPLES`
  samples (`LAG = RATE/5` = 200 ms, so late packets still land where they
  were heard), pulling from whichever queue has them and using **0
  (silence)** for a quiet source — never a gap.
- Samples are summed as i32 and clamped (peaks clip, they don't wrap).
- A source running ahead of the mix by more than `MAX_QUEUE` samples
  (0.6 s) loses its oldest samples (drift protection).
- `drain()` at the end takes everything still waiting.

### 3.3 The thread and the timing

- Thread name `nexora-notes`; exists **only while recording** (created by
  `Recording::start`, joined by `stop`/`Drop`).
- COM initialised `COINIT_MULTITHREADED` on the thread.
- Wakes every **40 ms** (`POLL`); a 500 ms device buffer leaves room for a
  late wake-up.
- `Recording::start` waits up to **5 s** (`OPEN_TIMEOUT`) for the devices to
  open and answers `(mic_opened, pc_opened)`; failure deletes the folder and
  returns the `CaptureIssue`.
- Stop sets an `AtomicBool`; the thread drains, closes the devices, closes
  the open part and returns `Take { dir, parts, secs }`.

### 3.4 The WAV parts (`Parts`)

- Folder: `%LOCALAPPDATA%\Nexora\notes\<meeting>\` where `<meeting>` is
  `meeting-<unix seconds>`; beside the parts, `started.txt` holds the local
  start time `YYYY-MM-DD HH:MM` (used for the file name later).
- Files: `part-000.wav`, `part-001.wav`, … each **exactly** 300 s (9.6 MB)
  except the last, written with `nexora_core::ielts::WavWriter` (16 kHz mono
  s16le PCM, 44-byte header).
- A part cut short has length 0 in its header; the server reads such a part
  to its end.

## 4. Handing the meeting over (`Sender`)

Thread name `nexora-notes-send`, only while a meeting is handed over; COM
MTA; HTTP client `Client::uncached` (no cache), User-Agent
`Nexora/<version> (+https://github.com/dasunjlk/Nexora)`.

1. **Key**: `Authorization: Bearer <key>` where the key is this PC's Nexora
   link key (`services/secret::load(paths::nexora_link_path())` →
   `nexora::parse_link`). No key ⇒ `Problem::SignedOut`. (The self-test
   injects its own base URL + key through `Job.api`.)
2. **Start** — `POST {BASE}/island/notes`, body `{"lang":"auto"|"id"|"en"}`;
   the answer `{"id":"…","part_max":12582912}` gives the meeting id (⇒
   `Problem::Server` if unreadable). Errors map through `problem()`.
3. **Parts** — for each part *n*: `POST …/notes/<id>/part/<n>` with the WAV
   bytes (`audio/wav`); after each, `AppEvent::Notes(Reply::Sent{sent,parts})`
   drives the progress bar. Part uploads get **180 s** each; other calls
   15 s.
4. **Finish** — `POST …/notes/<id>/finish` body `{}`.
5. **Poll** — every 4 s: `GET …/notes/<id>` → status `open` | `working` |
   `done` | `failed`:
   - `done` → save (below), then `DELETE …/notes/<id>` (best effort — the
     server needn't keep anything for the day),
   - `failed` → its `error` maps to a `Problem`,
   - `open` → the finish was lost; send finish again,
   - give up after 20 minutes ⇒ `Problem::TooSlow`.
6. **Retries** — every call is tried up to 3 times (`TRIES`) with backoff
   `2, 4, 8 s`; a 5xx is retried, a 4xx is not (it will answer the same).
   Connection errors ⇒ `Problem::Offline`. If the server forgot the meeting
   (`no_notes` / 404), the **whole hand-over runs once more** from step 2.
7. **Saved** — the audio folder is deleted; `Reply::Saved` carries
   `Saved { notes, file, started }`.

## 5. The wire contract (`api.nexora.app`)

Base: `nexora_core::notes::BASE = nexora::API` (https://api.nexora.app),
path prefix `/island/notes`. All bodies are JSON objects; every string field
is whitespace-collapsed by the parsers; unknown fields are ignored.

| Call | Request | Success answer | Notes |
|---|---|---|---|
| `POST /island/notes` | `{"lang":"auto"}` | `{"id":"dxY","part_max":12582912}` | starts a meeting |
| `POST /island/notes/:id/part/:n` | WAV bytes, `Content-Type: audio/wav` | `{"next":2,"secs":137}` | `next` = the part the server wants next (re-send/recover) |
| `POST /island/notes/:id/finish` | `{}` | 2xx (body unused) | meeting complete |
| `GET /island/notes/:id` | — | status objects below | polled while writing |
| `DELETE /island/notes/:id` | — | any | client-initiated cleanup |

`GET` status objects:

```jsonc
{"status":"open","secs":12}                       // parts still wanted
{"status":"working","secs":12}                    // writing
{"status":"failed","secs":2,"error":"nothing_heard"}
{"status":"done","secs":137,"notes":{…},"transcript":[…]}
```

`notes` object (all fields optional, defaults empty/`"Meeting"`):

```jsonc
{
  "title": "Launch plan",
  "lang": "id",
  "summary": ["Rilis Jumat"],
  "decisions": ["Harga 49rb"],
  "actions": [ {"who":"Budi","what":"Kirim deck","when":"Kamis"} ],
  "topics":  [ {"title":"Harga","points":["49rb"]} ]
}
```

`transcript` lines: `[{"t":12.5,"who":"Speaker 1","text":"Halo semua"}]` —
`t` is seconds from the start (non-numeric ⇒ 0; empty text dropped).

### 5.1 Errors

Error bodies: `{"error":"…","retry_after":17}` (+ `Retry-After` header
honoured). Mapping (`nexora_core::notes::problem`):

| status | `error` | `Problem` | retryable |
|---|---|---|---|
| 401 | any | `SignedOut` | yes |
| 503 | `paused` | `Paused` — feature switched off server-side | yes |
| 429 | `daily_cap` | `DailyCap` — today's minutes used | yes |
| 413 | `too_long` | `TooLong` | **no** |
| 403 | `pro_required` | `ProRequired` — the server's word only; Nexora's client never gates on Pro (everything is free) | yes |
| — | `nothing_heard` | `NothingHeard` | **no** |
| — / 404 | `no_notes` / any 404 | `Gone` — server forgot; resend once | yes |
| — | `ai_busy`, any 429/503 | `Busy(Some(wait_s))` | yes |
| — | `ai_failed`, any 502 | `AiFailed` | yes |
| — | other | `Server` | yes |
| (no answer) | — | `Offline` | yes |

Plain-words titles/lines are in `Problem::title()/line()` (e.g. "Nova is
swamped — Your recording is kept. Try again in 17 seconds."). The
`Unfinished` problem ("Your last meeting isn't written up") is raised by the
app, not the wire: the app closed on a meeting before its notes were saved.

## 6. State machines

### 6.1 Page phases (`Phase` / `Taker`)

```
Idle ──started(mic?,pc?)──► Listening{since}
Listening ──stopped(parts,secs)──► Sending{sent,parts}
Sending ──Reply::Sent{sent,parts}──► Sending (progress)
Sending/Writing ──Reply::Writing──► Writing{since}      (first ask fixes `since`)
Writing/Sending ──Reply::Saved──► Ready (Taker.last = Saved)
any ──Reply::Failed / failed()──► Failed(Problem)
Ready/Failed ──reset() / throw away──► Idle
```

- `live()` = Listening | Sending | Writing — drives the closed-island
  activity and the "don't sleep" logic.
- `elapsed(now)` = `now − since` while listening, else `secs`.
- `next_second(now)` = seconds until the shown second ticks (the app wakes
  exactly then instead of polling).
- App close while live: the newest meeting folder is kept (`kept()`), older
  folders are deleted; on next start the page shows `Unfinished` with
  **Write it up now** / **Throw away**.

### 6.2 The recording thread

```
open mic + pc (5 s timeout) ──both fail──► CaptureIssue (folder deleted)
loop every 40 ms:
  read each source (a dead device closes; reopened at the 2 s device check)
  push both into the mixer
  mixer.take(clock) → truncate at MAX_SECS → Parts.write
  stop flag → drain, close parts, return Take
```

## 7. Output

### 7.1 File name (`file_name`)

`YYYY-MM-DD HH.MM <title>.md` in `Documents\Nexora\Notes\`
(`paths::notes_dir()`), e.g. `2026-10-29 14.05 Launch plan.md` — sorts by
date. The title is cleaned of `< > : " / \ | ? *` and control chars,
whitespace-collapsed, cut at 60 chars, trailing dots trimmed; empty ⇒
`Meeting`. A name already taken gets ` (2)`, ` (3)`, … The file is UTF-8
**with a BOM** so Notepad and Word read the dashes and non-Latin text right.

### 7.2 Markdown format (`markdown(notes, started, with_transcript)`)

```markdown
# <title>

Thu 29 Oct 2026 · 14:05 · 31 min

## Summary

- …

## Decisions            ← only if non-empty

- …

## Action items         ← only if non-empty

- [ ] Kirim deck (Budi, Kamis)
- [ ] Cek harga          ← no who/when ⇒ no parentheses

## Notes                ← only if non-empty ("topics")

### Harga

- 49rb

## Transcript           ← only if with_transcript && non-empty

**[0:00] Nova:** Halo semua…

---
Notes by Nova, in Nexora
```

- The header line: `<weekday d Mon yyyy> · <HH:MM> · <rounded minutes> min`.
- `Copy` puts `markdown(notes, started, false)` (no transcript) on the
  clipboard; the file always has the transcript.

## 8. Privacy

- The recording exists only on the PC (`%LOCALAPPDATA%\Nexora\notes\`) until
  Stop; it is deleted the moment the notes are saved (or on Throw away).
- The meeting audio goes to the Nexora notes server over HTTPS with the PC's
  account key; the server transcribes and writes the notes and keeps nothing
  once the client's `DELETE` lands (the client fetches and saves everything
  itself).
- The microphone and the PC's sound are captured **only while recording**;
  the send thread lives only until saved/failed. Switching the add-on off
  removes the tab and deletes leftover audio.
- `PRIVACY.md` lists every request the feature makes.

## 9. Storage summary

| Path | What |
|---|---|
| `%LOCALAPPDATA%\Nexora\notes\<meeting>\` | `part-NNN.wav` + `started.txt` (audio lives only here) |
| `%LOCALAPPDATA%\Nexora\nexora-link.bin` | the sealed account key the hand-over carries |
| `Documents\Nexora\Notes\*.md` | the notes (kept forever; the user's files) |
| `%APPDATA%\Nexora\settings.json` | `addons` (the add-on switch) |
| `NEXORA_HOME` | overrides both app-data roots (self-test) |

## 10. Tests

Pure-logic tests ship in `nexora_core::notes` (`cargo test -p nexora-core`),
and the app's side is exercised by the scripted self-test
(`nexora.exe --selftest <dir>`, scene `notes`) with a stub server:

- `the_mix_follows_the_clock_and_fills_a_quiet_source_with_silence` — clock-
  driven mixing, silence where a source is quiet, clipping at the sum peak.
- `a_source_running_ahead_loses_its_oldest_samples` — drift protection keeps
  the newest.
- `the_end_of_a_recording_takes_what_still_waits` — `drain()`.
- `addresses_and_answers` — URL building (`a b` ⇒ `a%20b`), bodies, parsers
  (including junk answers ⇒ `None`).
- `done_notes_are_read_whole_and_tidy` — the full `done` object, defaults,
  whitespace collapsing, junk fields dropped.
- `problems_in_plain_words` — the whole error table, retryability.
- `the_page_follows_the_meeting` — the `Taker` phase machine, `since`
  stability, elapsed/next-second/formatting.
- `file_names_sort_by_date_and_keep_to_what_windows_allows` — name cleaning
  and length caps.
- `markdown_has_every_part_that_has_something` — headings appear only for
  non-empty parts; transcript on/off; footer.
- Self-test scene `notes` (and the notes self-test server) records a few
  seconds from a generated tone, hands it over to a local stub server and
  screenshots every phase; the sender's live path is covered by
  `Job.api` injection (base + key), so CI needs no account.

## 11. Files (implementation map)

| File | Lines | Role |
|---|---|---|
| `crates/nexora-core/src/notes.rs` | ~680 | Mixer, contract parsers, Taker/Phase/Problem, markdown, file_name, tests |
| `crates/nexora/src/services/notes.rs` | ~580 | Recording (sources, parts), kept()/clear_audio(), Sender (upload + poll + save) |
| `crates/nexora/src/pages/notes.rs` | ~350 | the Notes tab: record/stop, meters, phases, results, Try again |
| `crates/nexora/src/app.rs` (`App::notes_*`) | — | commands → Recording/Sender, events → Taker, unfinished-meeting flow |
| `crates/nexora/src/services/mic.rs` | — | WASAPI endpoints (`open_endpoint`), `privacy_denies()` |
| `crates/nexora-core/src/ielts.rs` | — | `Downmix` (any device format → 16 kHz mono), `WavWriter`, `peak` |
| `crates/nexora/src/paths.rs` | — | `notes_audio_dir()`, `notes_dir()` |
| `crates/nexora/src/events.rs` | — | `AppEvent::Notes(Reply)`, `AppEvent::NotesStopped(CaptureIssue)` |
| `crates/nexora/src/commands.rs` | — | `Command::Notes(NotesCmd)` |
| `crates/nexora/src/activities.rs` | — | `Activity::Notes` (closed island, Nova listening) |
| `crates/nexora/src/mac/services/capture.rs` | — | macOS sources: input device + Core Audio process tap (macOS 14.2+) |
| `crates/nexora/src/mac/app_notes.rs` | — | the Mac app's side |

## 12. Rebrand deltas (upstream → Nexora)

| | Upstream | Nexora |
|---|---|---|
| API base | `https://api.nexora.app` (`crate::nexora::API`) | `https://api.nexora.app` (`crate::nexora::API`) |
| Auth key file | `%LOCALAPPDATA%\Nexora\nexora-link.bin` | `%LOCALAPPDATA%\Nexora\nexora-link.bin` |
| Notes output folder | `Documents\Nexora\Notes` | `Documents\Nexora\Notes` |
| Audio root | `%LOCALAPPDATA%\Nexora\notes\` | `%LOCALAPPDATA%\Nexora\notes\` |
| Threads | `nexora-notes`, `nexora-notes-send` | `nexora-notes`, `nexora-notes-send` |
| App-data override | `NEXORA_HOME` | `NEXORA_HOME` |
| Copy/persona | "Nova listens…", "Notes by Nova, in Nexora" | "Nova listens…", "Notes by Nova, in Nexora" |
| Server codename | nexora-notes-server | nexora notes server (contract unchanged apart from the base URL) |
