"""Compose the shared 'focus' pose: sitting with an open book held by both hands.

The source clips only have idle arms hanging at the sides. This helper rebuilds
the focus sheet from the already-composed sit frames, removes the old lower
arms, and adds two connected arms plus hands on the book edges.
"""
from __future__ import annotations

import importlib.util
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
FRAME = 200

STYLE = {
    "cutegirl": {
        "erase": ((30, 145, 63, 166), (137, 145, 170, 166)),
        "arms": (((57, 121), (46, 136), (59, 162)), ((143, 121), (154, 136), (141, 162))),
        "book": (54, 145, 146, 180),
    },
    "cuteboy": {
        "erase": ((27, 142, 58, 166), (142, 142, 173, 166)),
        "arms": (((60, 119), (52, 136), (59, 160)), ((140, 119), (148, 136), (141, 160))),
        "book": (55, 142, 145, 177),
    },
}


def rgb_to_hsv(rgb):
    r, g, b = rgb[..., 0], rgb[..., 1], rgb[..., 2]
    maxc = np.max(rgb, axis=-1)
    minc = np.min(rgb, axis=-1)
    delta = maxc - minc
    h = np.zeros_like(maxc)
    s = np.zeros_like(maxc)
    mask = delta > 1e-6
    mr = mask & (maxc == r)
    mg = mask & (maxc == g)
    mb = mask & (maxc == b)
    h[mr] = ((g[mr] - b[mr]) / delta[mr]) % 6
    h[mg] = (b[mg] - r[mg]) / delta[mg] + 2
    h[mb] = (r[mb] - g[mb]) / delta[mb] + 4
    return (h / 6) % 1, s, maxc


def hsv_rgb(h, s, v):
    from colorsys import hsv_to_rgb

    return tuple(int(c * 255) for c in hsv_to_rgb(h % 1, min(1, s), min(1, v))) + (255,)


def curve(draw, p0, p1, p2, fill, width):
    points = []
    for t in np.linspace(0, 1, 28):
        x = (1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * p1[0] + t * t * p2[0]
        y = (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * p1[1] + t * t * p2[1]
        points.append((float(x), float(y)))
    draw.line(points, fill=fill, width=width, joint="curve")
    for x, y in (points[0], points[-1]):
        radius = width / 2
        draw.ellipse((x - radius, y - radius, x + radius, y + radius), fill=fill)


def make_focus_frames(sit_frames, style, skin):
    config = STYLE[style]
    skin_color = hsv_rgb(skin["skin_h"], skin["skin_s"], skin["skin_v"])
    cover = hsv_rgb((skin["dress_h"] + 0.5) % 1, 0.45, 0.85)
    dark = tuple(max(0, channel - 30) for channel in cover[:3]) + (255,)
    frames = []
    for source in sit_frames:
        frame = source.convert("RGBA").copy()
        arr = np.asarray(frame).copy()
        hsv, saturation, value = rgb_to_hsv(arr[..., :3].astype(np.float32) / 255.0)
        skin_mask = (
            (np.abs(hsv - skin["skin_h"]) < 0.045)
            & (saturation > 0.3)
            & (value > 0.45)
            & (arr[..., 3] > 20)
        )
        yy, xx = np.indices(skin_mask.shape)
        for x0, y0, x1, y1 in config["erase"]:
            region = skin_mask & (xx >= x0) & (xx < x1) & (yy >= y0) & (yy < y1)
            arr[..., 3][region] = 0
        frame = Image.fromarray(arr, "RGBA")
        draw = ImageDraw.Draw(frame)
        for arm in config["arms"]:
            curve(draw, *arm, skin_color, 9)
        x0, y0, x1, y1 = config["book"]
        draw.rounded_rectangle((x0, y0, x1, y1), radius=6, fill=cover)
        draw.rounded_rectangle(
            (x0 + 6, y0 + 6, x1 - 6, y1 - 6),
            radius=4,
            fill=(255, 252, 246, 255),
        )
        draw.line(
            ((x0 + x1) // 2, y0 + 6, (x0 + x1) // 2, y1 - 6),
            fill=dark,
            width=2,
        )
        for line_y in (y0 + 14, y0 + 23, y0 + 31):
            draw.line(
                (x0 + 12, line_y, (x0 + x1) // 2 - 5, line_y),
                fill=(224, 206, 186, 255),
                width=1,
            )
            draw.line(
                ((x0 + x1) // 2 + 5, line_y, x1 - 12, line_y),
                fill=(224, 206, 186, 255),
                width=1,
            )
        for hand_x in (x0 + 5, x1 - 5):
            draw.ellipse(
                (hand_x - 7, y1 - 14, hand_x + 7, y1 + 2),
                fill=skin_color,
            )
        frames.append(frame)
    return frames


def palettes():
    path = ROOT / "scripts/cutegirl/generate.py"
    spec = importlib.util.spec_from_file_location("yuzu_cutegirl_generator", path)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return {skin["id"]: skin for skin in module.SKINS}


def main():
    root = ROOT / "frontend/public/assets/character"
    for style in ("cutegirl", "cuteboy"):
        for skin in palettes().values():
            sit_path = root / style / skin["id"] / "sit.webp"
            if not sit_path.exists():
                continue
            sheet = Image.open(sit_path).convert("RGBA")
            frames = [
                sheet.crop((index * FRAME, 0, (index + 1) * FRAME, FRAME))
                for index in range(sheet.width // FRAME)
            ]
            result = Image.new("RGBA", (FRAME * len(frames), FRAME), (0, 0, 0, 0))
            for index, frame in enumerate(make_focus_frames(frames, style, skin)):
                result.alpha_composite(frame, (index * FRAME, 0))
            result.save(root / style / skin["id"] / "focus.webp", format="WEBP", quality=90, method=6)
            print("focus", style, skin["id"])


if __name__ == "__main__":
    main()
