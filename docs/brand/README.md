# Nexora brand files

Nexora belongs to the Nova family. Nova is the orange cat of the Nexora logo.

| File | What |
|---|---|
| `nova-mark.svg`, `nova-mark-white.svg` | Nova's head in colour and in white, copied unchanged from the Nexora brand kit |
| `nexora-logo.svg`, `nexora-logo-512.png` | The logo: an orange tile, the island hanging from its top edge playing music (album cover and equalizer, as on the real island), the white Nova below |
| `art/nexora-sit.webp` | Nova sitting on the island while it plays music (the installer's welcome and finish picture) |
| `art/nexora-peek.webp` | Nova peeking over the island while it plays music (README, link preview) |
| `art/nexora-asleep.webp` | Nova asleep on the island while it plays music (the website's Nova theme section) |
| `art/poses/nova-*.webp` | Nova with each page's prop: music, focus, health, shelf, tasks, clipboard, today, system (a processor chip), games (a controller), prayer (a lantern) (README pictures and website) |
| `readme-hero.webp` | The README banner: a 200 % screenshot of the website's hero (`site/index.html`), so the README shows the site's type |
| `social-preview.png` | The card GitHub shows when the repo link is shared (Settings → General → Social preview) |

`python3 tools/make_brand.py` redraws the logo, `crates/nexora/res/nexora.ico` and everything in `installer/art` from these files.

Drawing Nova: the head is exactly the shape of the mark, the left ear (viewer's left) is folded forward with a darker underside, and the face has only the two eyes: no mouth, nose, whiskers or blush. Don't redraw Nova in another style.

These files are not covered by the MIT license that covers the code.
