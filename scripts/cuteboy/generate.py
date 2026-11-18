"""Build Yuzu Cute Boy character assets from the CC0 The Boy pack (pzUH)."""
from __future__ import annotations

import glob
import json
import math
import os
import re
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
SRC = Path(os.environ.get("SRC_DIR", ROOT / "scripts/cuteboy/source/png"))
OUT = Path(os.environ.get("OUT_DIR", ROOT / "frontend/public/assets/character/cuteboy"))
FRAME = 200
WIN = 520
WIN_X0 = -100
WIN_Y0 = -10
DEAD_X0 = 90
DEAD_Y0 = 0


def natural(files):
    def key(path):
        match = re.search(r"\((\d+)\)", path)
        return int(match.group(1)) if match else 0

    return sorted(files, key=key)


def load(pattern):
    return [Image.open(path).convert("RGBA") for path in natural(glob.glob(str(SRC / f"{pattern} (*).png")))]


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
    h = (h / 6) % 1
    s[mask] = delta[mask] / np.maximum(maxc[mask], 1e-6)
    v = maxc
    return np.stack([h, s, v], axis=-1)


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


def near(rgb, color, tolerance):
    target = np.array(color, dtype=np.float32) / 255.0
    return np.linalg.norm(rgb - target, axis=-1) <= tolerance / 255.0


def recolor(image, skin, dead=False):
    arr = np.asarray(image).astype(np.float32) / 255.0
    rgb, alpha = arr[..., :3], arr[..., 3]
    hsv = rgb_to_hsv(rgb)
    h, s, v = hsv[..., 0], hsv[..., 1], hsv[..., 2]
    yy = np.arange(image.height)[:, None]
    skin_mask = near(rgb, (254, 206, 144), 8) | near(rgb, (254, 182, 96), 8)
    hair_color = (h > 0.035) & (h < 0.085) & (s > 0.08) & (s < 0.32) & (v > 0.42) & (v < 0.62)
    shorts_color = (h > 0.035) & (h < 0.085) & (s > 0.08) & (s < 0.32) & (v > 0.25) & (v <= 0.42)
    hair_mask = hair_color if dead else hair_color & (yy < 390)
    shorts_mask = shorts_color if dead else (hair_color | shorts_color) & (yy >= 390)
    eye_mask = (h > 0.48) & (h < 0.60) & (s > 0.90) & (v > 0.94)
    shirt_mask = (h > 0.48) & (h < 0.60) & (s > 0.30) & (s < 0.88) & (v > 0.70) & ~eye_mask
    cap_mask = ((h < 0.035) | (h > 0.97)) & (s > 0.45) & (v > 0.75)
    out = hsv.copy()
    out[..., 0][hair_mask] = skin["hair_h"]
    out[..., 1][hair_mask] = skin["hair_s"]
    out[..., 2][hair_mask] = skin["hair_v"]
    out[..., 0][shorts_mask] = skin["hair_h"]
    out[..., 1][shorts_mask] = 0.25
    out[..., 2][shorts_mask] = 0.30
    out[..., 0][eye_mask] = skin["eye_h"]
    out[..., 1][eye_mask] = 1.0
    out[..., 2][eye_mask] = 1.0
    out[..., 0][shirt_mask] = skin["dress_h"]
    out[..., 1][shirt_mask] = skin["dress_s"]
    out[..., 2][shirt_mask] = np.where(v[shirt_mask] < 0.90, skin["dress_v"] * 0.78, skin["dress_v"])
    out[..., 0][cap_mask] = (skin["dress_h"] + 0.5) % 1
    out[..., 1][cap_mask] = 0.78
    out[..., 2][cap_mask] = 1.0
    out[..., 0][skin_mask] = skin["skin_h"]
    out[..., 1][skin_mask] = skin["skin_s"]
    out[..., 2][skin_mask] = np.clip(v[skin_mask] * skin["skin_v"], 0, 1)
    result = hsv_to_rgb(out)
    return Image.fromarray((np.concatenate([result, alpha[..., None]], axis=-1) * 255).clip(0, 255).astype(np.uint8), "RGBA")


def shadow(width=FRAME, height=FRAME, wide=False, alpha=34):
    image = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    box = (18, 182, 182, 198) if wide else (58, 184, 142, 198)
    draw.ellipse(box, fill=(80, 62, 52, alpha))
    return image.filter(ImageFilter.GaussianBlur(3))


def compose(image, dead=False, wide_shadow=False, shift_x=0, shift_y=0):
    window = Image.new("RGBA", (WIN, WIN), (0, 0, 0, 0))
    if dead:
        window.alpha_composite(image, (-DEAD_X0, -DEAD_Y0))
    else:
        window.alpha_composite(image, (-WIN_X0, -WIN_Y0))
    frame = Image.new("RGBA", (FRAME, FRAME), (0, 0, 0, 0))
    frame.alpha_composite(shadow(FRAME, FRAME, wide_shadow))
    small = window.resize((FRAME, FRAME), Image.Resampling.LANCZOS)
    if shift_x or shift_y:
        moved = Image.new("RGBA", (FRAME, FRAME), (0, 0, 0, 0))
        moved.alpha_composite(small, (shift_x, shift_y))
        small = moved
    frame.alpha_composite(small)
    return frame


def sample(frames, count, loop=True):
    if loop:
        return [frames[round(index * len(frames) / count)] for index in range(count)]
    return [frames[round(index * (len(frames) - 1) / (count - 1))] for index in range(count)]


def hide_below(image, cutoff):
    arr = np.asarray(image).copy()
    arr[cutoff:, :, 3] = 0
    return Image.fromarray(arr, "RGBA")


def paste_shift(image, dy):
    out = Image.new("RGBA", image.size, (0, 0, 0, 0))
    out.alpha_composite(image, (0, dy))
    return out


def draw_book(image, skin):
    from colorsys import hsv_to_rgb as h2r

    draw = ImageDraw.Draw(image)
    cover = tuple(int(x * 255) for x in h2r((skin["dress_h"] + 0.5) % 1, 0.45, 0.85)) + (255,)
    dark = tuple(max(0, c - 30) for c in cover[:3]) + (255,)
    skin_rgb = tuple(int(x * 255) for x in h2r(skin["skin_h"], 0.43 * skin["skin_s"], skin["skin_v"])) + (255,)
    draw.rounded_rectangle((120, 360, 260, 416), radius=10, fill=cover)
    draw.rounded_rectangle((128, 366, 252, 408), radius=6, fill=(255, 252, 246, 255))
    draw.line((190, 366, 190, 408), fill=dark, width=3)
    for y in (376, 386, 396):
        draw.line((138, y, 180, y), fill=(224, 206, 186, 255), width=2)
        draw.line((200, y, 242, y), fill=(224, 206, 186, 255), width=2)
    draw.ellipse((112, 386, 130, 404), fill=skin_rgb)
    draw.ellipse((250, 386, 268, 404), fill=skin_rgb)


def build_sit(idle, skin, book=False):
    from colorsys import hsv_to_rgb as h2r

    char = paste_shift(hide_below(idle, 434), 50)
    if book:
        draw_book(char, skin)
    channel = (skin["dress_h"] + 0.25) % 1
    color = tuple(int(x * 255) for x in h2r(channel, 0.28, 0.95)) + (255,)
    dark = tuple(int(x * 255) for x in h2r(channel, 0.32, 0.80)) + (255,)
    light = tuple(int(x * 255) for x in h2r(channel, 0.18, 0.99)) + (255,)
    frames = []
    for index, dy in enumerate((0, -1, -2, -1, 0, 1)):
        out = Image.new("RGBA", idle.size, (0, 0, 0, 0))
        cushion = Image.new("RGBA", idle.size, (0, 0, 0, 0))
        draw = ImageDraw.Draw(cushion)
        draw.ellipse((20, 410, 300, 500), fill=color)
        draw.ellipse((42, 404, 278, 450), fill=light)
        draw.ellipse((20, 468, 300, 500), fill=dark)
        out.alpha_composite(cushion)
        moved = Image.new("RGBA", idle.size, (0, 0, 0, 0))
        moved.alpha_composite(char, (0, dy))
        out.alpha_composite(moved)
        front = ImageDraw.Draw(out)
        front.ellipse((42, 466, 278, 500), fill=color)
        front.ellipse((42, 486, 278, 500), fill=dark)
        frames.append(out)
    return frames


def build_droop(idle):
    frames = []
    for index, (angle, dy) in enumerate(((-6, 3), (-7, 4), (-8, 5), (-7, 4), (-6, 3), (-5, 2))):
        rotated = idle.rotate(angle, resample=Image.Resampling.BICUBIC, center=(158, 500), expand=False)
        out = Image.new("RGBA", idle.size, (0, 0, 0, 0))
        out.alpha_composite(rotated, (0, dy))
        frames.append(out)
    return frames


def draw_z(draw, x, y, size, fill, width):
    draw.line([(x, y), (x + size, y)], fill=fill, width=width)
    draw.line([(x + size, y), (x, y + size)], fill=fill, width=width)
    draw.line([(x, y + size), (x + size, y + size)], fill=fill, width=width)


def draw_zzz(image, index):
    draw = ImageDraw.Draw(image)
    drift = (0, 2, 4, 2)[index % 4]
    for x, y, size in ((68, 96 - drift, 14), (88, 70 - drift, 10), (103, 50 - drift, 7)):
        for ox, oy in ((-1, 0), (1, 0), (0, -1), (0, 1)):
            draw_z(draw, x + ox, y + oy, size, (255, 255, 255, 235), max(2, size // 5))
        draw_z(draw, x, y, size, (96, 76, 66, 245), max(2, size // 5))


def draw_star(draw, x, y, radius, fill):
    points = []
    for index in range(10):
        angle = -math.pi / 2 + index * math.pi / 5
        length = radius if index % 2 == 0 else radius * 0.42
        points.append((x + math.cos(angle) * length, y + math.sin(angle) * length))
    draw.polygon(points, fill=fill)


def draw_stars(image, index):
    draw = ImageDraw.Draw(image)
    pulse = index % 4
    for x, y, radius, alpha in ((42, 42, 7, 255), (162, 60, 6, 220), (120, 22, 5, 200)):
        draw_star(draw, x, y, radius + (1 if pulse == 1 else 0), (255, 210, 110, alpha))


def draw_exclaim(image, index):
    draw = ImageDraw.Draw(image)
    alpha = max(0, 255 - index * 30)
    if alpha <= 0:
        return
    draw.rounded_rectangle((140, 20, 150, 50), radius=5, fill=(255, 255, 255, alpha))
    draw.rounded_rectangle((142, 22, 148, 46), radius=3, fill=(236, 110, 90, alpha))
    draw.ellipse((142, 54, 149, 61), fill=(236, 110, 90, alpha))


def draw_wave(image, index):
    draw = ImageDraw.Draw(image)
    if index % 4 in (0, 1):
        for box in ((118, 106, 148, 144), (124, 114, 150, 152)):
            draw.arc(box, -60, 20, fill=(255, 255, 255, 235), width=6)
            draw.arc(box, -60, 20, fill=(240, 120, 90, 240), width=3)


def draw_sweat(image):
    draw = ImageDraw.Draw(image)
    draw.ellipse((144, 46, 158, 66), fill=(150, 210, 235, 235))
    draw.polygon([(148, 52), (152, 40), (156, 52)], fill=(150, 210, 235, 235))
    draw.ellipse((148, 50, 152, 55), fill=(235, 250, 255, 220))


def build_skin(skin, sources):
    sid = skin["id"]
    out = OUT / sid
    out.mkdir(parents=True, exist_ok=True)
    idle_frames = [recolor(frame, skin) for frame in sources["idle"]]
    walk_frames = [recolor(frame, skin) for frame in sources["walk"]]
    jump_frames = [recolor(frame, skin) for frame in sources["jump"]]
    dead_frames = [recolor(frame, skin, dead=True) for frame in sources["dead"][7:15]]
    sit_frames = build_sit(idle_frames[0], skin)
    focus_frames = build_sit(idle_frames[0], skin, book=True)
    droop_frames = build_droop(idle_frames[0])
    states = {}

    def add(name, frames, fps, loop, dead=False, wide=False, deco=None, shift=None):
        images = []
        for index, frame in enumerate(frames):
            sx, sy = shift(index) if shift else (0, 0)
            image = compose(frame, dead=dead, wide_shadow=wide, shift_x=sx, shift_y=sy)
            if deco:
                deco(image, index)
            images.append(image)
        states[name] = {"frames": images, "fps": fps, "loop": loop}

    add("idle", idle_frames, 10, True)
    add("walk", walk_frames, 14, True)
    add("sit", sit_frames, 4, True)
    add("sleep", dead_frames, 4, True, dead=True, wide=True, deco=draw_zzz)
    add("focus", focus_frames, 4, True)
    add("break", jump_frames, 8, True)
    add("celebrate", jump_frames, 12, False, deco=draw_stars)
    add("drag", jump_frames[3:13], 10, True, shift=lambda index: (int(3 * math.sin(index * 0.9)), 0))
    add("react", jump_frames[:10], 14, False, deco=draw_exclaim)
    add("remind", jump_frames[3:13], 8, True, deco=draw_wave)
    add("droop", droop_frames, 4, True, deco=lambda image, _: draw_sweat(image))

    manifest = {
        "character": sid,
        "displayName": skin["display"],
        "version": 4,
        "frameSize": [FRAME, FRAME],
        "anchor": [0.5, 1.0],
        "hitbox": [0.15, 0.05, 0.7, 0.9],
        "defaultState": "idle",
        "states": {},
    }
    for name, state in states.items():
        images = state["frames"]
        strip = Image.new("RGBA", (FRAME * len(images), FRAME), (0, 0, 0, 0))
        for index, image in enumerate(images):
            strip.alpha_composite(image, (index * FRAME, 0))
        strip.save(out / f"{name}.webp", format="WEBP", quality=90, method=6)
        manifest["states"][name] = {
            "file": f"{name}.webp",
            "frames": len(images),
            "fps": state["fps"],
            "loop": state["loop"],
        }
    (out / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    preview = states["idle"]["frames"][0]
    box = preview.getbbox()
    if box:
        x0, y0, x1, y1 = box
        side = max(x1 - x0, y1 - y0) + 4
        cx, cy = (x0 + x1) / 2, (y0 + y1) / 2
        crop = (int(cx - side / 2), int(cy - side / 2), int(cx + side / 2), int(cy + side / 2))
        square = Image.new("RGBA", (side, side), (0, 0, 0, 0))
        square.alpha_composite(preview.crop(crop), (0, 0))
        square.resize((256, 256), Image.Resampling.LANCZOS).save(out / "preview.png")
    print("built", sid)


SKINS = [
    dict(id="honey", display="Honey", hair_h=0.11, hair_s=1.00, hair_v=1.00, dress_h=0.11, dress_s=0.90, dress_v=1.10, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="cream", display="Cream", hair_h=0.13, hair_s=0.45, hair_v=1.00, dress_h=0.10, dress_s=0.55, dress_v=1.20, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="mint", display="Mint", hair_h=0.43, hair_s=0.90, hair_v=1.00, dress_h=0.45, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.35),
    dict(id="peach", display="Peach", hair_h=0.05, hair_s=1.00, hair_v=1.00, dress_h=0.04, dress_s=0.95, dress_v=1.10, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="night", display="Night", hair_h=0.65, hair_s=1.00, hair_v=0.38, dress_h=0.65, dress_s=0.90, dress_v=0.75, skin_h=0.08, skin_s=1.05, skin_v=0.90, eye_h=0.75),
    dict(id="rose", display="Rose", hair_h=0.93, hair_s=0.90, hair_v=1.00, dress_h=0.93, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="sky", display="Sky", hair_h=0.58, hair_s=0.90, hair_v=1.00, dress_h=0.58, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="lavender", display="Lavender", hair_h=0.74, hair_s=0.90, hair_v=1.00, dress_h=0.74, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.75),
    dict(id="jade", display="Jade", hair_h=0.38, hair_s=0.90, hair_v=0.95, dress_h=0.40, dress_s=0.90, dress_v=1.00, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.35),
    dict(id="coral", display="Coral", hair_h=0.995, hair_s=1.00, hair_v=1.00, dress_h=0.50, dress_s=1.00, dress_v=1.00, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="silver", display="Silver", hair_h=0.60, hair_s=0.15, hair_v=0.98, dress_h=0.60, dress_s=0.35, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="ocean", display="Ocean", hair_h=0.55, hair_s=1.00, hair_v=0.55, dress_h=0.52, dress_s=1.00, dress_v=0.85, skin_h=0.08, skin_s=1.05, skin_v=0.90, eye_h=0.55),
    dict(id="berry", display="Berry", hair_h=0.88, hair_s=0.95, hair_v=0.90, dress_h=0.88, dress_s=0.90, dress_v=1.00, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.75),
    dict(id="cocoa", display="Cocoa", hair_h=0.07, hair_s=1.00, hair_v=0.55, dress_h=0.05, dress_s=0.90, dress_v=0.75, skin_h=0.06, skin_s=1.10, skin_v=0.75, eye_h=0.55),
    dict(id="snow", display="Snow", hair_h=0.55, hair_s=0.12, hair_v=1.00, dress_h=0.55, dress_s=0.35, dress_v=1.20, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id="sunny", display="Sunny", hair_h=0.13, hair_s=1.00, hair_v=1.00, dress_h=0.08, dress_s=1.00, dress_v=1.10, skin_h=0.08, skin_s=1.05, skin_v=0.92, eye_h=0.55),
]


def main():
    sources = {name.lower(): load(name) for name in ("Idle", "Walk", "Jump", "Dead")}
    print({name: len(frames) for name, frames in sources.items()})
    only = os.environ.get("ONLY")
    for skin in SKINS:
        if only and skin["id"] != only:
            continue
        build_skin(skin, sources)


if __name__ == "__main__":
    main()
