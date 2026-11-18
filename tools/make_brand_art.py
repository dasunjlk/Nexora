#!/usr/bin/env python3
"""Draw the website's Nova art in docs/brand/art: the three island scenes
(nexora-peek, nexora-sit, nexora-asleep) and the ten page poses, flat, in
Nexora's violets — the same cat server/make_phone_art.py draws.

  nexora-peek.webp   Nova peeking over the island while a song plays (hero)
  nexora-sit.webp    Nova sitting on the island (installer, page pictures)
  nexora-asleep.webp Nova asleep on the island (the Nova theme section)
  poses/nova-*.webp  Nova with each page's prop

Run: python3 tools/make_brand_art.py   (needs Pillow)
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "server"))
from make_phone_art import (  # noqa: E402
    FUR, FUR_DARK, INNER, INK, SS, canvas, head, paw, rrect, ellipse, tail,
)
from PIL import Image, ImageDraw  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs" / "brand" / "art"


def save(img: Image.Image, w: int, h: int, name: str):
    img.resize((w, h), Image.LANCZOS).save(OUT / name, quality=90, method=4)
    print(f"art/{name}  {w}x{h}")

ISLAND = "#0A0A0A"
ACCENT = "#7C5CFF"
ACCENT_HI = "#9D85FF"
PORCELAIN = "#F3F0FF"
PAPER = "#FFFFFF"


def island(d, w, h, x0, y0, wpx, hpx, playing=True):
    """The island: a black pill with an album dot and equalizer bars."""
    rrect(d, [x0, y0, x0 + wpx, y0 + hpx], hpx / 2, fill=ISLAND)
    if not playing:
        return
    dot = hpx * 0.42
    dxy = y0 + hpx * 0.29
    ellipse(d, [x0 + hpx * 0.28, dxy, x0 + hpx * 0.28 + dot, dxy + dot], fill=ACCENT_HI)
    ellipse(d, [x0 + hpx * 0.28 + dot * 0.3, dxy + dot * 0.28, x0 + hpx * 0.28 + dot * 0.8, dxy + dot * 0.72], fill="#2B2B33")
    for i, k in enumerate((0.45, 0.85, 0.6, 1.0)):
        bh = hpx * 0.34 * k
        bx = x0 + wpx - hpx * 0.75 - i * hpx * 0.14
        rrect(d, [bx, y0 + hpx / 2 - bh / 2, bx + hpx * 0.07, y0 + hpx / 2 + bh / 2], hpx * 0.035, fill=ACCENT)


def peek(w, h):
    img, d, _ = canvas(w, h)
    bar_y = h * 0.52
    island(d, w, h, w * 0.10, bar_y, w * 0.80, h * 0.34)
    head(d, w * 0.5, bar_y - h * 0.075, h * 0.082)
    for sx in (-1, 1):
        paw(d, w * 0.5 + sx * w * 0.062, bar_y + h * 0.02, h * 0.033)
    save(img, w, h, "nexora-peek.webp")


def sit(w, h):
    img, d, _ = canvas(w, h)
    cx = w * 0.5
    top = h * 0.62
    island(d, w, h, w * 0.06, top, w * 0.88, h * 0.16)
    # Nova sits on the island: body above it, tail down its front
    body_top = top - h * 0.185
    rrect(d, [cx - w * 0.19, body_top + h * 0.06, cx + w * 0.19, top + h * 0.035], w * 0.17, fill=FUR)
    head(d, cx, body_top - h * 0.005, w * 0.13)
    for sx in (-1, 1):
        paw(d, cx + sx * w * 0.115, top - h * 0.02, w * 0.07)
    d.arc([(cx + w * 0.14) * SS, (top - h * 0.1) * SS, (cx + w * 0.30) * SS, (top + h * 0.16) * SS],
          250, 90, fill=FUR_DARK, width=round(h * 0.02 * SS))
    save(img, w, h, "nexora-sit.webp")


def asleep(w, h):
    img, d, _ = canvas(w, h)
    top = h * 0.55
    island(d, w, h, w * 0.08, top, w * 0.84, h * 0.20)
    # curled on top of the island, eyes closed
    cx, cy = w * 0.46, top - h * 0.045
    ellipse(d, [cx - w * 0.22, cy - h * 0.10, cx + w * 0.24, cy + h * 0.11], fill=FUR)
    head(d, cx - w * 0.155, cy - h * 0.075, w * 0.105, eyes="closed")
    for i, (zx, zy, r) in enumerate([(0.63, 0.30, 0.028), (0.70, 0.20, 0.04), (0.79, 0.09, 0.055)]):
        zr = r * w
        ellipse(d, [w * zx - zr, h * zy - zr * 0.55, w * zx + zr, h * zy + zr * 0.55], outline=INK, width=round(w * 0.012 * SS))
    save(img, w, h, "nexora-asleep.webp")


def base(w, h):
    """A sitting Nova, centred, for the page poses. Returns (img, d, cx, ground)."""
    img, d, _ = canvas(w, h)
    cx = w * 0.5
    ground = h * 0.94
    body_h = h * 0.26
    rrect(d, [cx - w * 0.26, ground - body_h, cx + w * 0.26, ground + body_h * 0.14], w * 0.2, fill=FUR)
    head(d, cx, ground - body_h - h * 0.115, w * 0.155)
    return img, d, cx, ground, body_h


def prop_card(d, x, y, w, h, lines=3, r=None):
    rrect(d, [x, y, x + w, y + h], (r or h * 0.08), fill=PAPER)
    for i in range(lines):
        ly = y + h * (0.26 + i * 0.2)
        d.line([(x + w * 0.14) * SS, ly * SS, (x + w * (0.86 if i % 2 == 0 else 0.6)) * SS, ly * SS],
               fill=ACCENT, width=round(h * 0.045 * SS))


def pose_music(w, h):
    img, d, cx, ground, body_h = base(w, h)
    # headphones over the ears
    hy = ground - body_h - h * 0.115
    r = w * 0.19
    d.arc([(cx - r) * SS, (hy - r) * SS, (cx + r) * SS, (hy + r * 0.9) * SS], 180, 360, fill=INK, width=round(w * 0.045 * SS))
    for sx in (-1, 1):
        rrect(d, [cx + sx * r - w * 0.055, hy - h * 0.01, cx + sx * r + w * 0.055, hy + h * 0.075], w * 0.045, fill=ACCENT)
    # a couple of notes
    for nx, ny, s in [(0.20, 0.24, 1.0), (0.30, 0.16, 0.75)]:
        nr = w * 0.032 * s
        ellipse(d, [w * nx - nr, h * ny - nr, w * nx + nr, h * ny + nr], fill=ACCENT)
        d.line([(w * nx + nr) * SS, (h * ny) * SS, (w * nx + nr) * SS, (h * ny - h * 0.11 * s) * SS], fill=ACCENT, width=round(w * 0.02 * s * SS))
    save(img, w, h, "poses/nova-music.webp")


def pose_today(w, h):
    img, d, cx, ground, body_h = base(w, h)
    px, pw, ph = cx - w * 0.17, w * 0.34, h * 0.24
    py = ground - body_h * 0.72
    rrect(d, [px, py, px + pw, py + ph], h * 0.03, fill=PAPER)
    rrect(d, [px, py, px + pw, py + ph * 0.28], h * 0.03, fill=ACCENT)
    for r in range(2):
        for c in range(4):
            dot = ph * 0.075
            dx = px + pw * (0.16 + c * 0.22) - dot / 2
            dy = py + ph * (0.45 + r * 0.28) - dot / 2
            filled = (r * 4 + c) % 5 == 0
            ellipse(d, [dx, dy, dx + dot, dy + dot], fill=ACCENT if filled else "#C9C4E8")
    save(img, w, h, "poses/nova-today.webp")


def pose_shelf(w, h):
    img, d, cx, ground, body_h = base(w, h)
    bx, bw = cx - w * 0.42, w * 0.36
    for i, col in enumerate([ACCENT, ACCENT_HI, "#5b3fd6"]):
        by = ground - h * 0.14 - i * h * 0.052 - h * 0.052
        rrect(d, [bx, by, bx + bw, by + h * 0.048], h * 0.02, fill=col)
    rrect(d, [bx, ground - h * 0.14, bx + bw, ground], h * 0.025, fill="#2A2438")
    save(img, w, h, "poses/nova-shelf.webp")


def pose_tasks(w, h):
    img, d, cx, ground, body_h = base(w, h)
    px, pw, ph = cx - w * 0.17, w * 0.34, h * 0.24
    py = ground - body_h * 0.72
    rrect(d, [px, py, px + pw, py + ph], h * 0.03, fill=PAPER)
    for i in range(3):
        ty = py + ph * (0.24 + i * 0.26)
        box = ph * 0.13
        rrect(d, [px + pw * 0.13, ty - box / 2, px + pw * 0.13 + box, ty + box / 2], box * 0.25,
              outline=ACCENT, width=round(h * 0.014 * SS))
        if i < 2:  # the done ones carry a check
            d.line([(px + pw * 0.165) * SS, ty * SS, (px + pw * 0.20) * SS, (ty + box * 0.2) * SS], fill=ACCENT, width=round(h * 0.014 * SS))
            d.line([(px + pw * 0.20) * SS, (ty + box * 0.2) * SS, (px + pw * 0.27) * SS, (ty - box * 0.24) * SS], fill=ACCENT, width=round(h * 0.014 * SS))
        d.line([(px + pw * 0.36) * SS, ty * SS, (px + pw * 0.85) * SS, ty * SS], fill="#C9C4E8", width=round(h * 0.02 * SS))
    save(img, w, h, "poses/nova-tasks.webp")


def pose_focus(w, h):
    img, d, cx, ground, body_h = base(w, h)
    r = w * 0.15
    cxx, cy = cx, ground - body_h * 0.55
    d.arc([(cxx - r) * SS, (cy - r) * SS, (cxx + r) * SS, (cy + r) * SS], 0, 360, fill="#C9C4E8", width=round(w * 0.055 * SS))
    d.arc([(cxx - r) * SS, (cy - r) * SS, (cxx + r) * SS, (cy + r) * SS], 250, 375, fill=ACCENT, width=round(w * 0.055 * SS))
    ellipse(d, [cxx - w * 0.02, cy - w * 0.02, cxx + w * 0.02, cy + w * 0.02], fill=INK)
    save(img, w, h, "poses/nova-focus.webp")


def pose_health(w, h):
    img, d, cx, ground, body_h = base(w, h)
    cxx, cy, r = cx, ground - body_h * 0.5, w * 0.075
    for a, b, rr in [(0.0, 1.0, 1.0), (-0.09, 0.91, 1.0)]:
        d.polygon([((cxx) * SS, (cy + r * 0.95) * SS),
                   ((cxx - r * 1.25) * SS, (cy - r * 0.35) * SS),
                   ((cxx - r * 0.6) * SS, (cy - r * 1.15) * SS),
                   ((cxx) * SS, (cy - r * 0.45) * SS),
                   ((cxx + r * 0.6) * SS, (cy - r * 1.15) * SS),
                   ((cxx + r * 1.25) * SS, (cy - r * 0.35) * SS)], fill=ACCENT if a == 0.0 else "#9d85ff")
    save(img, w, h, "poses/nova-health.webp")


def pose_system(w, h):
    img, d, cx, ground, body_h = base(w, h)
    cxx, cy = cx, ground - body_h * 0.5
    s = w * 0.11
    rrect(d, [cxx - s, cy - s, cxx + s, cy + s], s * 0.22, fill="#2A2438")
    rrect(d, [cxx - s * 0.62, cy - s * 0.62, cxx + s * 0.62, cy + s * 0.62], s * 0.12, fill=ACCENT)
    for i in range(4):
        off = -s * 0.6 + i * s * 0.4
        d.line([(cxx + off) * SS, (cy - s) * SS, (cxx + off) * SS, (cy - s * 1.35) * SS], fill="#2A2438", width=round(s * 0.12 * SS))
        d.line([(cxx + off) * SS, (cy + s) * SS, (cxx + off) * SS, (cy + s * 1.35) * SS], fill="#2A2438", width=round(s * 0.12 * SS))
        d.line([(cxx - s) * SS, (cy + off) * SS, (cxx - s * 1.35) * SS, (cy + off) * SS], fill="#2A2438", width=round(s * 0.12 * SS))
        d.line([(cxx + s) * SS, (cy + off) * SS, (cxx + s * 1.35) * SS, (cy + off) * SS], fill="#2A2438", width=round(s * 0.12 * SS))
    save(img, w, h, "poses/nova-system.webp")


def pose_prayer(w, h):
    img, d, cx, ground, body_h = base(w, h)
    cxx, cy = cx, ground - body_h * 0.5
    lw, lh = w * 0.115, h * 0.155
    rrect(d, [cxx - lw, cy - lh * 0.6, cxx + lw, cy + lh * 0.75], lw * 0.3, fill=ACCENT)
    ellipse(d, [cxx - lw * 0.95, cy - lh * 0.15, cxx + lw * 0.95, cy + lh * 0.35], fill="#F3F0FF")
    rrect(d, [cxx - lw * 0.55, cy - lh * 0.95, cxx + lw * 0.55, cy - lh * 0.55], lw * 0.2, fill=FUR_DARK)
    d.line([(cxx) * SS, (cy - lh * 1.15) * SS, (cxx) * SS, (cy - lh * 0.9) * SS], fill=FUR_DARK, width=round(w * 0.02 * SS))
    save(img, w, h, "poses/nova-prayer.webp")


def pose_games(w, h):
    img, d, cx, ground, body_h = base(w, h)
    cxx, cy = cx, ground - body_h * 0.5
    pw, ph = w * 0.21, h * 0.115
    rrect(d, [cxx - pw, cy - ph, cxx + pw, cy + ph], ph * 0.75, fill="#2A2438")
    for i, (sx, sy, col) in enumerate([(-0.5, -0.35, ACCENT), (-0.15, 0.1, ACCENT_HI), (0.45, -0.2, "#5b3fd6"), (0.75, 0.25, "#9d85ff")]):
        br = ph * 0.17
        ellipse(d, [cxx + sx * pw - br, cy + sy * ph - br, cxx + sx * pw + br, cy + sy * ph + br], fill=col)
    d.line([(cxx - pw * 0.75) * SS, cy * SS, (cxx - pw * 0.45) * SS, cy * SS], fill="#F3F0FF", width=round(ph * 0.1 * SS))
    d.line([(cxx - pw * 0.60) * SS, (cy - ph * 0.15) * SS, (cxx - pw * 0.60) * SS, (cy + ph * 0.15) * SS], fill="#F3F0FF", width=round(ph * 0.1 * SS))
    save(img, w, h, "poses/nova-games.webp")


def pose_clipboard(w, h):
    img, d, cx, ground, body_h = base(w, h)
    px, pw, ph = cx - w * 0.16, w * 0.32, h * 0.23
    py = ground - body_h * 0.72
    rrect(d, [px, py, px + pw, py + ph], h * 0.025, fill="#2A2438")
    rrect(d, [px + pw * 0.08, py + ph * 0.06, px + pw * 0.92, py + ph * 0.94], h * 0.02, fill=PAPER)
    rrect(d, [cx - pw * 0.12, py - ph * 0.06, cx + pw * 0.12, py + ph * 0.10], h * 0.012, fill=ACCENT)
    for i in range(3):
        ly = py + ph * (0.34 + i * 0.2)
        d.line([(px + pw * 0.2) * SS, ly * SS, (px + pw * 0.8) * SS, ly * SS], fill="#C9C4E8", width=round(h * 0.02 * SS))
    save(img, w, h, "poses/nova-clipboard.webp")


def social_card():
    """The repository's link preview: the logo tile, the name and the tag."""
    w, h = 1280, 640
    img = Image.new("RGBA", (w * SS, h * SS), (23, 23, 23, 255))
    d = ImageDraw.Draw(img)
    tile = Image.open(ROOT / "docs" / "brand" / "nexora-logo-512.png").convert("RGBA")
    side = h * 0.62
    art = tile.resize((round(side * SS), round(side * SS)), Image.LANCZOS)
    img.alpha_composite(art, (round(w * 0.115 * SS), round((h - side) / 2 * SS)))
    font_bold = str(ROOT / "crates" / "nexora" / "res" / "fonts" / "Outfit-Bold.ttf")
    font_med = str(ROOT / "crates" / "nexora" / "res" / "fonts" / "Outfit-Medium.ttf")
    from PIL import ImageFont
    name = ImageFont.truetype(font_bold, round(h * 0.20 * SS))
    tag = ImageFont.truetype(font_med, round(h * 0.055 * SS))
    d.text((w * 0.415 * SS, h * 0.36 * SS), "Nexora", font=name, fill=(255, 255, 255, 255))
    d.text((w * 0.42 * SS, h * 0.60 * SS), "A living notch for Windows and Mac", font=tag, fill=(180, 155, 255, 255))
    img.resize((w, h), Image.LANCZOS).save(ROOT / "social-preview.png", quality=95)
    print("social-preview.png  1280x640")


def main():
    OUT.mkdir(exist_ok=True)
    (OUT / "poses").mkdir(exist_ok=True)
    peek(1200, 661)
    sit(893, 1200)
    asleep(900, 654)
    pose_music(480, 560)
    pose_today(480, 499)
    pose_shelf(480, 660)
    pose_tasks(480, 577)
    pose_focus(480, 466)
    pose_health(480, 572)
    pose_system(480, 602)
    pose_prayer(480, 562)
    pose_games(480, 492)
    pose_clipboard(480, 477)
    social_card()


if __name__ == "__main__":
    main()
