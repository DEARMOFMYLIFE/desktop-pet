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
        "erase": (),
        "arms": (((52, 146), (55, 152), (59, 158)), ((148, 146), (145, 152), (141, 158))),
        "book": (54, 148, 146, 181),
    },
    "cuteboy": {
        "erase": (),
        "arms": (((48, 143), (53, 149), (59, 156)), ((152, 143), (147, 149), (141, 156))),
        "book": (55, 145, 145, 178),
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
    s[mask] = delta[mask] / np.maximum(maxc[mask], 1e-6)
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



def sample_color(frames, hue, saturation=(0.2, 0.9), value=(0.5, 1.0)):
    samples = []
    for image in frames:
        rgba = np.asarray(image.convert("RGBA"))
        hsv, s, v = rgb_to_hsv(rgba[..., :3].astype(np.float32) / 255.0)
        mask = (
            (np.abs(hsv - hue) < 0.04)
            & (s > saturation[0])
            & (s < saturation[1])
            & (v > value[0])
            & (v < value[1])
            & (rgba[..., 3] > 80)
        )
        if mask.any():
            samples.append(rgba[..., :3][mask])
    if not samples:
        return None
    pixels = np.concatenate(samples, axis=0)
    return tuple(int(channel) for channel in np.median(pixels, axis=0)) + (255,)


def make_focus_frames(sit_frames, style, skin):
    config = STYLE[style]
    skin_color = sample_color(sit_frames, skin["skin_h"], (0.2, 0.95), (0.6, 1.05))
    if skin_color is None:
        skin_color = hsv_rgb(skin["skin_h"], 0.43, skin["skin_v"])
    clothing = sample_color(sit_frames, skin["dress_h"], (0.2, 0.95), (0.45, 1.05))
    if clothing is None:
        clothing = hsv_rgb(skin["dress_h"], 0.55, skin["dress_v"])
    cover = tuple(int(channel * 0.82) for channel in clothing[:3]) + (255,)
    dark = tuple(max(0, int(channel * 0.62)) for channel in clothing[:3]) + (255,)
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
        # Put the hands exactly on the arm endpoints, then let them overlap
        # the book edge. This makes the grip explicit rather than a separate
        # pair of floating mitts at the bottom of the cover.
        for _, _, (hand_x, hand_y) in config["arms"]:
            draw.ellipse(
                (hand_x - 7, hand_y - 7, hand_x + 7, hand_y + 7),
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
