# Nexora brand files

Nova is Nexora's cat: violet fur (`#7C5CFF`, fold `#5B3FD6`), drawn from
vectors so she stays sharp at any scaling. Everything here is generated —
redraw it with the tools below after changing the palette or the geometry.

| File | What | Drawn by |
|---|---|---|
| `nexora-logo.svg`, `nexora-logo-512.png` | the logo: a violet tile with the island hanging from its top edge playing music (album cover and equalizer, as on the real island), and the white Nova below | `tools/make_brand.py` |
| `nova-mark.svg`, `nova-mark-white.svg` | Nova's head in colour and in white | the brand kit's geometry; the fills follow `nexora_core::nova`'s palette |
| `art/nexora-peek.webp` | Nova peeking over the island while a song plays (the website's hero) | `tools/make_brand_art.py` |
| `art/nexora-sit.webp` | Nova sitting on the island (the installer's welcome picture, a page picture) | `tools/make_brand_art.py` |
| `art/nexora-asleep.webp` | Nova asleep on the island (the Nova theme section) | `tools/make_brand_art.py` |
| `art/poses/nova-*.webp` | Nova with each page's prop: music (headphones), today (calendar), shelf (files), tasks (checklist), focus (timer ring), health (heart), system (chip), prayer (lantern), games (controller), clipboard | `tools/make_brand_art.py` |
| `social-preview.png` | the card GitHub shows when the repo link is shared (Settings → General → Social preview) | `tools/make_brand_art.py` |

The app's icon and the installer's pictures come from the same generators:
`crates/nexora/res/nexora.ico` and `installer/art/*` are `tools/make_brand.py`'s
output; the phone page's icons and illustrations are `server/make_icons.py`'s
and `server/make_phone_art.py`'s.

```
python -I tools/make_brand.py
python -I tools/make_brand_art.py
python -I server/make_icons.py
python -I server/make_phone_art.py
```

The palette lives in `docs/design.md`; the app's own drawing constants are
`nexora_core::nova` (`FUR`, `FOLD`, `INK`) and `nexora_core::look` (the Nova
theme).

Drawing Nova: the head is exactly the shape of the mark, the left ear
(viewer's left) is folded forward with a darker underside, and the face has
only the two eyes: no mouth, nose, whiskers or blush. Don't redraw Nova in
another style.

These files, the Nexora name and the Nova character are not covered by the
Apache-2.0 license that covers the code.
