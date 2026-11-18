"""Turn the native sit animation into a calm reading pose.

The character keeps its original, anti-aliased sprite frames. Only a smooth
book and a soft contact shadow are composited on the lap; no arms or hands are
painted over the sprite, so the result stays close to the source artwork.
"""
from __future__ import annotations

import importlib.util
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
FRAME = 200
SCALE = 4

BOOK = {
    "cutegirl": (60, 155, 140, 185),
    "cuteboy": (60, 152, 140, 182),
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


def book_overlay(style, clothing):
    x0, y0, x1, y1 = (value * SCALE for value in BOOK[style])
    overlay = Image.new("RGBA", (FRAME * SCALE, FRAME * SCALE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(overlay)
    cover = tuple(int(channel * 0.82) for channel in clothing[:3]) + (255,)
    border = tuple(int(channel * 0.62) for channel in clothing[:3]) + (255,)

    draw.rounded_rectangle(
        (x0 + 4 * SCALE, y1 - 3 * SCALE, x1 - 4 * SCALE, y1 + 5 * SCALE),
        radius=8 * SCALE,
        fill=(80, 62, 52, 35),
    )
    draw.rounded_rectangle((x0, y0, x1, y1), radius=6 * SCALE, fill=cover)
    draw.rounded_rectangle(
        (x0 + 6 * SCALE, y0 + 6 * SCALE, x1 - 6 * SCALE, y1 - 6 * SCALE),
        radius=4 * SCALE,
        fill=(255, 252, 246, 255),
    )
    draw.line(
        ((x0 + x1) // 2, y0 + 6 * SCALE, (x0 + x1) // 2, y1 - 6 * SCALE),
        fill=border,
        width=2 * SCALE,
    )
    for line_y in (y0 + 14 * SCALE, y0 + 23 * SCALE, y0 + 31 * SCALE):
        draw.line(
            (x0 + 12 * SCALE, line_y, (x0 + x1) // 2 - 5 * SCALE, line_y),
            fill=(224, 206, 186, 255),
            width=SCALE,
        )
        draw.line(
            ((x0 + x1) // 2 + 5 * SCALE, line_y, x1 - 12 * SCALE, line_y),
            fill=(224, 206, 186, 255),
            width=SCALE,
        )
    return overlay.resize((FRAME, FRAME), Image.Resampling.LANCZOS)


def palettes():
    path = ROOT / "scripts/cutegirl/generate.py"
    spec = importlib.util.spec_from_file_location("yuzu_cutegirl_generator", path)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return {skin["id"]: skin for skin in module.SKINS}


def make_focus_frames(sit_frames, style, skin):
    clothing = sample_color(sit_frames, skin["dress_h"], (0.2, 0.95), (0.45, 1.05))
    if clothing is None:
        clothing = hsv_rgb(skin["dress_h"], 0.55, skin["dress_v"])
    overlay = book_overlay(style, clothing)
    frames = []
    for source in sit_frames:
        frame = source.convert("RGBA").copy()
        frame.alpha_composite(overlay)
        frames.append(frame)
    return frames


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