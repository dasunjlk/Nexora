#!/usr/bin/env python3
"""Draw the phone page's Nova illustrations in server/public.

Nova is Nexora's cat, drawn flat in the brand's violets. Five pictures, at
the sizes the page's HTML carries:

  nova-peek.webp    her eyes and paws over the island's edge
  nova-sit.webp     sitting, tail curled round
  nova-shelf.webp   sitting beside the Shelf's files
  nova-asleep.webp  curled up asleep, mid-dream
  nova-note.webp    holding the meeting notes

Run: python3 server/make_phone_art.py   (needs Pillow)
"""
from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "server" / "public"

FUR = "#7C5CFF"
FUR_DARK = "#5B3FD6"
INNER = "#A78BFA"
INK = "#171717"
FIELD = "#EFEAFF"
WHITE = "#FFFFFF"
SS = 4  # supersample


def canvas(w: int, h: int) -> tuple[Image.Image, ImageDraw.ImageDraw, float]:
    img = Image.new("RGBA", (w * SS, h * SS), (0, 0, 0, 0))
    return img, ImageDraw.Draw(img), SS


def rrect(d: ImageDraw.ImageDraw, box, r, **kw):
    d.rounded_rectangle([c * SS for c in box], radius=r * SS, **kw)


def ellipse(d: ImageDraw.ImageDraw, box, **kw):
    d.ellipse([c * SS for c in box], **kw)


def head(d: ImageDraw.ImageDraw, cx: float, cy: float, r: float, eyes="open", tilt=0.0):
    """The head: a rounded skull with two ears, the fold cut in."""
    hw = r * 1.18  # half width
    hh = r * 1.02
    # ears
    for sx in (-1, 1):
        ex = cx + sx * hw * 0.62
        ey = cy - hh * 0.62
        ear = [(ex - r * 0.44, ey - r * 0.34), (ex + r * 0.46, ey + r * 0.16), (ex - r * 0.02, ey + r * 0.40)]
        d.polygon([(x * SS, y * SS) for x, y in ear], fill=FUR)
        gx = sum(p[0] for p in ear) / 3
        gy = sum(p[1] for p in ear) / 3
        d.polygon(
            [((gx + (x - gx) * 0.55) * SS, (gy + (y - gy) * 0.55) * SS) for x, y in ear],
            fill=INNER,
        )
    # skull: a squashed circle with cheeks
    ellipse(d, [cx - hw, cy - hh, cx + hw, cy + hh], fill=FUR)
    ellipse(d, [cx - hw * 0.98, cy - hh * 0.2, cx + hw * 0.98, cy + hh * 1.02], fill=FUR)
    # eyes
    for sx in (-1, 1):
        ex = cx + sx * hw * 0.42
        ey = cy + hh * 0.08
        if eyes == "open":
            ellipse(d, [ex - r * 0.13, ey - r * 0.16, ex + r * 0.13, ey + r * 0.16], fill=INK)
        else:  # closed: a small arc
            d.arc(
                [(ex - r * 0.14) * SS, (ey - r * 0.14) * SS, (ex + r * 0.14) * SS, (ey + r * 0.18) * SS],
                20 if sx < 0 else 160,
                160 if sx < 0 else 340,
                fill=INK,
                width=round(r * 0.09 * SS),
            )
    # nose and mouth
    nx, ny = cx, cy + hh * 0.42
    ellipse(d, [nx - r * 0.07, ny - r * 0.05, nx + r * 0.07, ny + r * 0.05], fill=INK)
    d.arc(
        [(nx - r * 0.16) * SS, (ny) * SS, nx * SS, (ny + r * 0.14) * SS],
        0, 110, fill=INK, width=round(r * 0.055 * SS),
    )
    d.arc(
        [nx * SS, (ny) * SS, (nx + r * 0.16) * SS, (ny + r * 0.14) * SS],
        70, 180, fill=INK, width=round(r * 0.055 * SS),
    )


def paw(d: ImageDraw.ImageDraw, cx: float, cy: float, r: float):
    ellipse(d, [cx - r, cy - r * 0.72, cx + r, cy + r * 0.72], fill=FUR)
    for k in (-0.28, 0, 0.28):
        d.line([(cx + k * r * 0.6) * SS, (cy - r * 0.3) * SS, (cx + k * r * 0.6) * SS, (cy + r * 0.34) * SS], fill=FUR_DARK, width=round(r * 0.09 * SS))


def tail(d: ImageDraw.ImageDraw, cx: float, cy: float, r: float, flip=1):
    # A curled tail: an arc whose box is mirrored for a cat facing the other way.
    if flip >= 0:
        box = [cx - r * 1.5, cy - r, cx + r * 0.2, cy + r * 1.7]
    else:
        box = [cx - r * 0.2, cy - r, cx + r * 1.5, cy + r * 1.7]
    d.arc([c * SS for c in box], 240, 60, fill=FUR_DARK, width=round(r * 0.34 * SS))


def save(img: Image.Image, w: int, h: int, name: str):
    img.resize((w, h), Image.LANCZOS).save(OUT / name, quality=90, method=4)
    print(f"{name}  {w}x{h}")


def peek(w: int, h: int):
    """Eyes and paws over the island's edge."""
    img, d, _ = canvas(w, h)
    bar = h * 0.42
    rrect(d, [w / 2 - w * 0.30, bar, w / 2 + w * 0.30, h], h * 0.06, fill="#0A0A0A")
    head(d, w / 2, bar - h * 0.115, h * 0.115)
    for sx in (-1, 1):
        paw(d, w / 2 + sx * w * 0.115, bar + h * 0.045, h * 0.05)
    save(img, w, h, "nova-peek.webp")


def sit(w: int, h: int):
    """Sitting, tail curled round her feet."""
    img, d, _ = canvas(w, h)
    cx = w * 0.5
    ground = h * 0.88
    body_h = h * 0.34
    rrect(d, [cx - w * 0.24, ground - body_h, cx + w * 0.24, ground + body_h * 0.18], w * 0.22, fill=FUR)
    ellipse(d, [cx - w * 0.20, ground - body_h * 0.62, cx - w * 0.02, ground + body_h * 0.12], fill=FUR_DARK)
    head(d, cx, ground - body_h - h * 0.115, w * 0.17)
    for sx in (-1, 1):
        paw(d, cx + sx * w * 0.13, ground + body_h * 0.02, w * 0.075)
    tail(d, cx + w * 0.20, ground - body_h * 0.35, w * 0.16, flip=-1)
    save(img, w, h, "nova-sit.webp")


def shelf(w: int, h: int):
    """Sitting beside the Shelf: a stack of files."""
    img, d, _ = canvas(w, h)
    cx = w * 0.60
    ground = h * 0.88
    body_h = h * 0.30
    rrect(d, [cx - w * 0.21, ground - body_h, cx + w * 0.21, ground + body_h * 0.18], w * 0.19, fill=FUR)
    head(d, cx, ground - body_h - h * 0.10, w * 0.15)
    for sx in (-1, 1):
        paw(d, cx + sx * w * 0.115, ground + body_h * 0.02, w * 0.07)
    tail(d, cx + w * 0.18, ground - body_h * 0.30, w * 0.14, flip=-1)
    # the Shelf: three file cards
    bx = w * 0.10
    bw = w * 0.34
    for i, (dh, col) in enumerate([(h * 0.055, "#8B6CFF"), (h * 0.055, "#B49BFF"), (h * 0.055, "#6E4BE8")]):
        y = ground - h * 0.16 - i * dh - dh
        rrect(d, [bx, y, bx + bw, y + dh * 0.92], h * 0.02, fill=col)
    rrect(d, [bx, ground - h * 0.16, bx + bw, ground], h * 0.025, fill="#2A2438")
    save(img, w, h, "nova-shelf.webp")


def asleep(w: int, h: int):
    """Curled up, eyes closed, dreaming."""
    img, d, _ = canvas(w, h)
    cx = w * 0.46
    cy = h * 0.68
    ellipse(d, [cx - w * 0.28, cy - h * 0.14, cx + w * 0.30, cy + h * 0.18], fill=FUR)
    d.arc(
        [(cx + w * 0.16) * SS, (cy - h * 0.14) * SS, (cx + w * 0.36) * SS, (cy + h * 0.18) * SS],
        250, 90, fill=FUR_DARK, width=round(h * 0.05 * SS),
    )
    head(d, cx - w * 0.20, cy - h * 0.13, w * 0.14, eyes="closed")
    for i, (zx, zy, r) in enumerate([(0.70, 0.42, 0.03), (0.78, 0.30, 0.043), (0.87, 0.17, 0.06)]):
        zr = r * w
        ellipse(d, [w * zx - zr, h * zy - zr * 0.55, w * zx + zr, h * zy + zr * 0.55], outline=INK, width=round(w * 0.014 * SS))
    save(img, w, h, "nova-asleep.webp")


def note(w: int, h: int):
    """Holding the meeting notes."""
    img, d, _ = canvas(w, h)
    cx = w * 0.5
    ground = h * 0.94
    body_h = h * 0.24
    rrect(d, [cx - w * 0.26, ground - body_h, cx + w * 0.26, ground + body_h * 0.14], w * 0.2, fill=FUR)
    head(d, cx, ground - body_h - h * 0.115, w * 0.155)
    # the notepad, held below her chin
    px, pw, ph = cx - w * 0.17, w * 0.34, h * 0.24
    py = ground - body_h * 0.72
    rrect(d, [px, py, px + pw, py + ph], h * 0.03, fill=WHITE)
    for i in range(3):
        y = py + ph * 0.30 + i * ph * 0.2
        d.line([(px + pw * 0.14) * SS, y * SS, (px + pw * (0.86 if i % 2 == 0 else 0.6)) * SS, y * SS], fill="#8B6CFF", width=round(h * 0.013 * SS))
    for sx in (-1, 1):
        paw(d, cx + sx * pw * 0.42, py + ph * 0.16, w * 0.07)
    save(img, w, h, "nova-note.webp")


def main() -> None:
    peek(640, 353)
    sit(420, 564)
    shelf(320, 440)
    asleep(560, 407)
    note(320, 318)


if __name__ == "__main__":
    main()
