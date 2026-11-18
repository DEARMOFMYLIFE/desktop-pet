"""Build the original robot and cat hero styles from CC0 pzUH packs."""
from __future__ import annotations

import json
import re
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "frontend/public/assets/character"
FRAME = 200


def natural(paths):
    def key(path):
        match = re.search(r"\((\d+)\)", path.name)
        return int(match.group(1)) if match else 0

    return sorted(paths, key=key)


def load_state(folder, name):
    return [Image.open(path).convert("RGBA") for path in natural(folder.glob(f"{name} (*).png"))]


def compose(frames):
    boxes = [frame.getbbox() for frame in frames if frame.getbbox()]
    if not boxes:
        return []
    x0 = min(box[0] for box in boxes)
    y0 = min(box[1] for box in boxes)
    x1 = max(box[2] for box in boxes)
    y1 = max(box[3] for box in boxes)
    side = int(max(x1 - x0, y1 - y0) * 1.12)
    cx, cy = (x0 + x1) / 2, (y0 + y1) / 2
    box = (int(cx - side / 2), int(cy - side / 2), int(cx + side / 2), int(cy + side / 2))
    output = []
    for frame in frames:
        square = Image.new("RGBA", (side, side), (0, 0, 0, 0))
        square.alpha_composite(frame.crop(box))
        output.append(square.resize((FRAME, FRAME), Image.Resampling.LANCZOS))
    return output


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


def hsv_to_rgb(hsv):
    h, s, v = hsv[..., 0], hsv[..., 1], hsv[..., 2]
    h6 = (h % 1) * 6
    i = np.floor(h6).astype(np.int32) % 6
    f = h6 - np.floor(h6)
    p = v * (1 - s)
    q = v * (1 - f * s)
    t = v * (1 - (1 - f) * s)
    out = np.zeros(hsv.shape[:-1] + (3,), dtype=np.float32)
    for index, (rr, gg, bb) in enumerate(((v, t, p), (q, v, p), (p, v, t), (p, q, v), (t, p, v), (v, p, q))):
        mask = i == index
        out[mask, 0] = rr[mask]
        out[mask, 1] = gg[mask]
        out[mask, 2] = bb[mask]
    return out


def tint(frame, hue_shift=0.0, saturation=1.0, value=1.0, warm_gray=False):
    rgba = np.asarray(frame.convert("RGBA")).astype(np.float32) / 255.0
    hue, s, v = rgb_to_hsv(rgba[..., :3])
    hue = (hue + hue_shift) % 1
    s = np.clip(s * saturation, 0, 1)
    v = np.clip(v * value, 0, 1)
    if warm_gray:
        gray = s < 0.16
        hue[gray] = (0.055 + hue_shift) % 1
        s[gray] = np.clip(s[gray] + 0.42, 0, 0.7)
    hsv = np.stack([hue, s, v], axis=-1)
    rgb = hsv_to_rgb(hsv)
    return Image.fromarray((np.concatenate([rgb, rgba[..., 3:4]], axis=-1) * 255).clip(0, 255).astype(np.uint8), "RGBA")


def write_style(style, source, mapping, variants):
    states = {name: compose(load_state(source, source_name)) for name, source_name in mapping.items()}
    if not all(states.values()):
        raise RuntimeError(f"missing source frames for {style}")
    for skin_id, options in variants.items():
        out = OUT / style / skin_id
        out.mkdir(parents=True, exist_ok=True)
        manifest = {
            "character": style,
            "displayName": skin_id,
            "version": 4,
            "frameSize": [FRAME, FRAME],
            "anchor": [0.5, 1.0],
            "hitbox": [0.15, 0.05, 0.7, 0.9],
            "defaultState": "idle",
            "states": {},
        }
        rendered = {}
        for name, frames in states.items():
            frames = [tint(frame, **options) for frame in frames]
            rendered[name] = frames
            strip = Image.new("RGBA", (FRAME * len(frames), FRAME), (0, 0, 0, 0))
            for index, frame in enumerate(frames):
                strip.alpha_composite(frame, (index * FRAME, 0))
            strip.save(out / f"{name}.webp", format="WEBP", quality=90, method=6)
        fps = {
            "idle": 10, "walk": 14, "sit": 4, "sleep": 4, "focus": 4,
            "break": 8, "celebrate": 12, "drag": 10, "react": 14, "remind": 8, "droop": 4,
        }
        for name, frames in rendered.items():
            manifest["states"][name] = {
                "file": f"{name}.webp", "frames": len(frames),
                "fps": fps[name], "loop": name not in {"celebrate", "react"},
            }
        (out / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        preview = rendered["idle"][0]
        box = preview.getbbox()
        if box:
            x0, y0, x1, y1 = box
            side = max(x1 - x0, y1 - y0) + 4
            cx, cy = (x0 + x1) / 2, (y0 + y1) / 2
            crop = (int(cx - side / 2), int(cy - side / 2), int(cx + side / 2), int(cy + side / 2))
            square = Image.new("RGBA", (side, side), (0, 0, 0, 0))
            square.alpha_composite(preview.crop(crop), (0, 0))
            square.resize((256, 256), Image.Resampling.LANCZOS).save(out / "preview.png")
        print("built", style, skin_id)


def main():
    robot = ROOT / "scripts/robot/source/png"
    cat = ROOT / "scripts/cat/source/png/cat"
    robot_map = {
        "idle": "Idle", "walk": "Run", "sit": "Idle", "sleep": "Dead", "focus": "Idle",
        "break": "Jump", "celebrate": "Jump", "drag": "Jump", "react": "JumpMelee",
        "remind": "Shoot", "droop": "Slide",
    }
    cat_map = {
        "idle": "Idle", "walk": "Walk", "sit": "Idle", "sleep": "Dead", "focus": "Idle",
        "break": "Jump", "celebrate": "Jump", "drag": "Jump", "react": "Hurt",
        "remind": "Jump", "droop": "Slide",
    }
    robot_variants = {
        "coral": {"hue_shift": 0.0},
        "honey": {"hue_shift": 0.04, "saturation": 1.08},
        "mint": {"hue_shift": 0.24, "saturation": 0.95},
        "sky": {"hue_shift": 0.52, "saturation": 0.95},
        "lavender": {"hue_shift": 0.72, "saturation": 0.9},
        "rose": {"hue_shift": 0.93, "saturation": 0.95},
        "night": {"hue_shift": 0.62, "saturation": 0.7, "value": 0.72},
        "silver": {"hue_shift": 0.55, "saturation": 0.25, "value": 1.08},
    }
    cat_variants = {
        "coral": {"hue_shift": 0.0, "warm_gray": True, "saturation": 1.0},
        "honey": {"hue_shift": 0.06, "warm_gray": True, "saturation": 1.15},
        "mint": {"hue_shift": 0.35, "warm_gray": True, "saturation": 0.95},
        "sky": {"hue_shift": 0.55, "warm_gray": True, "saturation": 1.0},
        "lavender": {"hue_shift": 0.75, "warm_gray": True, "saturation": 0.95},
        "rose": {"hue_shift": 0.92, "warm_gray": True, "saturation": 1.0},
        "night": {"hue_shift": 0.65, "warm_gray": True, "saturation": 0.7, "value": 0.75},
        "silver": {"saturation": 0.8},
    }
    write_style("robot", robot, robot_map, robot_variants)
    write_style("cat", cat, cat_map, cat_variants)


if __name__ == "__main__":
    main()