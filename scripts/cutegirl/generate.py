"""Build Yuzu character assets from the CC0 Cute Girl pack (pzUH)."""
from __future__ import annotations
from PIL import Image, ImageDraw, ImageFilter
import numpy as np
import glob, json, math, os, re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SRC = Path(os.environ.get('SRC_DIR', ROOT / 'scripts/cutegirl/source/png'))
OUT = Path(os.environ.get('OUT_DIR', ROOT / 'frontend/public/assets/character'))
FRAME = 200
WIN = 480
WIN_Y0 = -26
WIN_X0 = -20
DEAD_X0 = 143
DEAD_Y0 = 22

def natural(files):
    def key(p):
        m = re.search(r'\((\d+)\)', p)
        return int(m.group(1)) if m else 0
    return sorted(files, key=key)

def load(pat):
    return [Image.open(p).convert('RGBA') for p in natural(glob.glob(str(SRC / f'{pat} (*).png')))]

def rgb_to_hsv(rgb):
    r, g, b = rgb[..., 0], rgb[..., 1], rgb[..., 2]
    maxc = np.max(rgb, axis=-1); minc = np.min(rgb, axis=-1)
    delta = maxc - minc
    h = np.zeros_like(maxc); s = np.zeros_like(maxc)
    mask = delta > 1e-6
    mr = mask & (maxc == r); mg = mask & (maxc == g); mb = mask & (maxc == b)
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
    p = v * (1 - s); q = v * (1 - f * s); t = v * (1 - (1 - f) * s)
    out = np.zeros(hsv.shape[:-1] + (3,), dtype=np.float32)
    for idx, (rr, gg, bb) in enumerate([(v,t,p),(q,v,p),(p,v,t),(p,q,v),(t,p,v),(v,p,q)]):
        m = i == idx
        out[m, 0] = rr[m]; out[m, 1] = gg[m]; out[m, 2] = bb[m]
    return out

def eye_boxes(src: Image.Image):
    arr = np.asarray(src).astype(np.float32) / 255.0
    rgb, alpha = arr[..., :3], arr[..., 3]
    hsv = rgb_to_hsv(rgb)
    h, s, v = hsv[..., 0], hsv[..., 1], hsv[..., 2]
    mask = (h > 0.48) & (h < 0.60) & (s > 0.75) & (v > 0.75) & (alpha > 0.5)
    H, W = mask.shape
    seen = np.zeros_like(mask, dtype=bool)
    boxes = []
    for y in range(H):
        for x in range(W):
            if not mask[y, x] or seen[y, x]:
                continue
            stack = [(y, x)]
            seen[y, x] = True
            minx = maxx = x
            miny = maxy = y
            count = 0
            while stack:
                cy, cx = stack.pop()
                count += 1
                minx = min(minx, cx); maxx = max(maxx, cx)
                miny = min(miny, cy); maxy = max(maxy, cy)
                for ny, nx in ((cy-1,cx),(cy+1,cx),(cy,cx-1),(cy,cx+1)):
                    if 0 <= ny < H and 0 <= nx < W and mask[ny, nx] and not seen[ny, nx]:
                        seen[ny, nx] = True
                        stack.append((ny, nx))
            if count > 30:
                boxes.append((minx, miny, maxx + 1, maxy + 1))
    return boxes

def patch_eyes(im: Image.Image, boxes) -> Image.Image:
    im = im.copy()
    d = ImageDraw.Draw(im)
    for x0, y0, x1, y1 in boxes:
        cx = (x0 + x1) // 2
        sy = min(y1 + 9, im.height - 1)
        sample = im.getpixel((cx, sy))
        skin = sample[:3] + (255,) if sample[3] > 0 else (255, 206, 150, 255)
        bx0, by0, bx1, by1 = x0 - 6, y0 - 6, x1 + 6, y1 + 6
        d.ellipse((bx0, by0, bx1, by1), fill=skin)
        cy = (y0 + y1) // 2
        d.line([(bx0 + 4, cy), (cx, cy + 3), (bx1 - 4, cy)],
               fill=(72, 48, 42, 255), width=2, joint='curve')
    return im

def recolor(im: Image.Image, skin: dict) -> Image.Image:
    arr = np.asarray(im).astype(np.float32) / 255.0
    rgb, alpha = arr[..., :3], arr[..., 3]
    hsv = rgb_to_hsv(rgb)
    h, s, v = hsv[..., 0], hsv[..., 1], hsv[..., 2]
    yy = np.arange(im.height)[:, None]
    hair = ((h < 0.04) | (h > 0.96)) & (s > 0.42) & (v > 0.5)
    eyes = (h >= 0.48) & (h <= 0.60) & (s > 0.7) & (v > 0.85)
    dress = (h >= 0.44) & (h <= 0.56) & (s >= 0.18) & (s <= 0.55) & (v >= 0.3) & (v <= 0.85) & ~eyes
    skin_m = (h >= 0.05) & (h <= 0.14) & (s >= 0.3) & (v >= 0.8) & ~dress & ~eyes
    out = hsv.copy()
    def apply(mask, hue, sm, vm):
        out[..., 0][mask] = hue
        out[..., 1][mask] = np.clip(s[mask] * sm, 0, 1)
        out[..., 2][mask] = np.clip(v[mask] * vm, 0, 1)
    apply(hair, skin['hair_h'], skin['hair_s'], skin['hair_v'])
    apply(dress, skin['dress_h'], skin['dress_s'], skin['dress_v'])
    apply(skin_m, skin['skin_h'], skin['skin_s'], skin['skin_v'])
    out[..., 0][eyes] = skin['eye_h']
    rgb2 = hsv_to_rgb(out)
    final = np.concatenate([rgb2, alpha[..., None]], axis=-1)
    return Image.fromarray((final * 255).clip(0, 255).astype(np.uint8), 'RGBA')

def shadow(w=200, h=200, wide=False, alpha=34):
    im = Image.new('RGBA', (w, h), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    if wide:
        d.ellipse((18, 182, 182, 198), fill=(80, 62, 52, alpha))
    else:
        d.ellipse((58, 184, 142, 198), fill=(80, 62, 52, alpha))
    return im.filter(ImageFilter.GaussianBlur(3))

def compose(im: Image.Image, dead=False, wide_shadow=False, shift_x=0, shift_y=0) -> Image.Image:
    win = Image.new('RGBA', (WIN, WIN), (0, 0, 0, 0))
    win.alpha_composite(im, (-DEAD_X0, -DEAD_Y0) if dead else (-WIN_X0, -WIN_Y0))
    frame = Image.new('RGBA', (FRAME, FRAME), (0, 0, 0, 0))
    frame.alpha_composite(shadow(FRAME, FRAME, wide_shadow))
    small = win.resize((FRAME, FRAME), Image.Resampling.LANCZOS)
    if shift_x or shift_y:
        moved = Image.new('RGBA', (FRAME, FRAME), (0, 0, 0, 0))
        moved.alpha_composite(small, (shift_x, shift_y))
        small = moved
    frame.alpha_composite(small)
    return frame

def sample(frames, n, loop=True):
    if loop:
        return [frames[round(i * len(frames) / n)] for i in range(n)]
    return [frames[round(i * (len(frames) - 1) / (n - 1))] for i in range(n)]

def remove_legs(im: Image.Image) -> Image.Image:
    arr = np.asarray(im).astype(np.float32) / 255.0
    rgb, alpha = arr[..., :3], arr[..., 3]
    hsv = rgb_to_hsv(rgb)
    s, v = hsv[..., 1], hsv[..., 2]
    yy = np.arange(im.height)[:, None]
    legs = (s < 0.08) & (v > 0.85) & (yy > 390) & (alpha > 0.01)
    alpha[legs] = 0
    final = np.concatenate([rgb, alpha[..., None]], axis=-1)
    return Image.fromarray((final * 255).clip(0, 255).astype(np.uint8), 'RGBA')

def paste_shift(im: Image.Image, dy: int) -> Image.Image:
    out = Image.new('RGBA', im.size, (0, 0, 0, 0))
    out.alpha_composite(im, (0, dy))
    return out

def draw_book(im: Image.Image, skin: dict) -> None:
    from colorsys import hsv_to_rgb as h2r
    d = ImageDraw.Draw(im)
    cover = tuple(int(x * 255) for x in h2r((skin['dress_h'] + 0.5) % 1, 0.45, 0.85)) + (255,)
    dark = tuple(max(0, c - 30) for c in cover[:3]) + (255,)
    skin_rgb = tuple(int(x * 255) for x in h2r(skin['skin_h'], 0.43 * skin['skin_s'], 1.0 * skin['skin_v'])) + (255,)
    d.rounded_rectangle((140, 378, 276, 434), radius=10, fill=cover)
    d.rounded_rectangle((148, 384, 268, 426), radius=6, fill=(255, 252, 246, 255))
    d.line((208, 384, 208, 426), fill=dark, width=3)
    for y in (394, 404, 414):
        d.line((158, y, 198, y), fill=(224, 206, 186, 255), width=2)
        d.line((218, y, 258, y), fill=(224, 206, 186, 255), width=2)
    d.ellipse((132, 404, 150, 422), fill=skin_rgb)
    d.ellipse((266, 404, 284, 422), fill=skin_rgb)

def draw_z(d, x, y, s, fill, width):
    d.line([(x, y), (x + s, y)], fill=fill, width=width)
    d.line([(x + s, y), (x, y + s)], fill=fill, width=width)
    d.line([(x, y + s), (x + s, y + s)], fill=fill, width=width)

def draw_zzz(im: Image.Image, fi: int) -> None:
    d = ImageDraw.Draw(im)
    drift = [0, 2, 4, 2][fi % 4]
    zs = [(68, 96 - drift, 14), (88, 70 - drift, 10), (103, 50 - drift, 7)]
    for x, y, s in zs:
        for ox, oy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            draw_z(d, x + ox, y + oy, s, (255, 255, 255, 235), max(2, s // 5))
        draw_z(d, x, y, s, (96, 76, 66, 245), max(2, s // 5))

def draw_star(d, x, y, r, fill):
    pts = []
    for i in range(10):
        a = -math.pi / 2 + i * math.pi / 5
        rad = r if i % 2 == 0 else r * 0.42
        pts.append((x + math.cos(a) * rad, y + math.sin(a) * rad))
    d.polygon(pts, fill=fill)

def draw_stars(im: Image.Image, fi: int) -> None:
    d = ImageDraw.Draw(im)
    pulse = fi % 4
    for x, y, r, a in [(42, 42, 7, 255), (162, 60, 6, 220), (120, 22, 5, 200)]:
        rr = r + (1 if pulse == 1 else 0)
        draw_star(d, x, y, rr, (255, 210, 110, a))

def draw_exclaim(im: Image.Image, fi: int) -> None:
    d = ImageDraw.Draw(im)
    a = max(0, 255 - fi * 30)
    if a <= 0:
        return
    d.rounded_rectangle((166, 28, 176, 58), radius=5, fill=(255, 255, 255, a))
    d.rounded_rectangle((168, 30, 174, 54), radius=3, fill=(236, 110, 90, a))
    d.ellipse((168, 62, 175, 69), fill=(236, 110, 90, a))

def draw_wave(im: Image.Image, fi: int) -> None:
    d = ImageDraw.Draw(im)
    if fi % 4 in (0, 1):
        for box in [(150, 128, 180, 166), (156, 136, 182, 174)]:
            d.arc(box, -60, 20, fill=(255, 255, 255, 235), width=6)
            d.arc(box, -60, 20, fill=(240, 120, 90, 240), width=3)

def draw_sweat(im: Image.Image) -> None:
    d = ImageDraw.Draw(im)
    d.ellipse((330, 142, 348, 166), fill=(150, 210, 235, 235))
    d.polygon([(334, 148), (339, 136), (344, 148)], fill=(150, 210, 235, 235))
    d.ellipse((334, 146, 339, 151), fill=(235, 250, 255, 220))

def build_sit(idle: Image.Image, blink: Image.Image, skin: dict, book=False):
    from colorsys import hsv_to_rgb as h2r
    char = remove_legs(idle)
    blink_char = remove_legs(blink)
    base = paste_shift(char, 50)
    blink_base = paste_shift(blink_char, 50)
    if book:
        draw_book(base, skin)
        draw_book(blink_base, skin)
    ch = (skin['dress_h'] + 0.25) % 1
    c = tuple(int(x * 255) for x in h2r(ch, 0.28, 0.95)) + (255,)
    cd = tuple(int(x * 255) for x in h2r(ch, 0.32, 0.80)) + (255,)
    cl = tuple(int(x * 255) for x in h2r(ch, 0.18, 0.99)) + (255,)
    cushion = Image.new('RGBA', idle.size, (0, 0, 0, 0))
    d = ImageDraw.Draw(cushion)
    d.ellipse((34, 406, 382, 462), fill=c)
    d.ellipse((54, 402, 362, 438), fill=cl)
    d.ellipse((34, 438, 382, 462), fill=cd)
    frames = []
    for idx, dy in enumerate([0, -1, -2, -1, 0, 1]):
        out = Image.new('RGBA', idle.size, (0, 0, 0, 0))
        out.alpha_composite(cushion)
        moved = Image.new('RGBA', idle.size, (0, 0, 0, 0))
        moved.alpha_composite(blink_base if idx == 3 else base, (0, dy))
        out.alpha_composite(moved)
        d2 = ImageDraw.Draw(out)
        d2.ellipse((58, 436, 358, 462), fill=c)
        d2.ellipse((58, 448, 358, 462), fill=cd)
        frames.append(out)
    return frames

def build_droop(idle: Image.Image, blink: Image.Image):
    frames = []
    for idx, (angle, dy) in enumerate([(-6, 3), (-7, 4), (-8, 5), (-7, 4), (-6, 3), (-5, 2)]):
        src = blink if idx == 4 else idle
        rot = src.rotate(angle, resample=Image.Resampling.BICUBIC, center=(208, 452), expand=False)
        out = Image.new('RGBA', idle.size, (0, 0, 0, 0))
        out.alpha_composite(rot, (0, dy))
        draw_sweat(out)
        frames.append(out)
    return frames

def build_skin(skin: dict, sources: dict):
    sid = skin['id']
    out = OUT / sid
    out.mkdir(parents=True, exist_ok=True)
    idle_src = sources['idle']
    walk_src = sources['walk']
    jump_src = sources['jump']
    dead_src = sources['dead']
    idle0 = recolor(idle_src[0], skin)
    blink0 = patch_eyes(recolor(idle_src[0], skin), eye_boxes(idle_src[0]))
    idle_frames = [recolor(f, skin) for f in idle_src]
    idle_frames[10] = patch_eyes(idle_frames[10], eye_boxes(idle_src[10]))
    walk_frames = [recolor(f, skin) for f in walk_src]
    jump_frames = [recolor(f, skin) for f in sample(jump_src, 16, loop=False)]
    dead_frames = [recolor(f, skin) for f in sample(dead_src[22:30], 8, loop=True)]
    sit_frames = build_sit(idle0, blink0, skin, book=False)
    focus_frames = build_sit(idle0, blink0, skin, book=True)
    droop_frames = build_droop(idle0, blink0)

    states = {}
    def add(name, frames, fps, loop, dead=False, wide=False, deco=None, shift=None):
        imgs = []
        for i, f in enumerate(frames):
            sx, sy = shift(i) if shift else (0, 0)
            im = compose(f, dead=dead, wide_shadow=wide, shift_x=sx, shift_y=sy)
            if deco:
                deco(im, i)
            imgs.append(im)
        states[name] = {'frames': imgs, 'fps': fps, 'loop': loop}

    add('idle', idle_frames, 10, True)
    add('walk', walk_frames, 14, True)
    add('sit', sit_frames, 4, True)
    add('sleep', dead_frames, 4, True, dead=True, wide=True, deco=lambda im, i: draw_zzz(im, i))
    add('focus', focus_frames, 4, True)
    add('break', jump_frames, 8, True)
    add('celebrate', jump_frames, 12, False, deco=lambda im, i: draw_stars(im, i))
    add('drag', jump_frames[3:13], 10, True, shift=lambda i: (int(3 * math.sin(i * 0.9)), 0))
    add('react', jump_frames[:10], 14, False, deco=lambda im, i: draw_exclaim(im, i))
    add('remind', jump_frames[3:13], 8, True, deco=lambda im, i: draw_wave(im, i))
    add('droop', droop_frames, 4, True)

    manifest = {
        'character': sid,
        'displayName': skin['display'],
        'version': 4,
        'frameSize': [FRAME, FRAME],
        'anchor': [0.5, 1.0],
        'hitbox': [0.15, 0.05, 0.7, 0.9],
        'defaultState': 'idle',
        'states': {},
    }
    for name, st in states.items():
        imgs = st['frames']
        strip = Image.new('RGBA', (FRAME * len(imgs), FRAME), (0, 0, 0, 0))
        for i, im in enumerate(imgs):
            strip.alpha_composite(im, (i * FRAME, 0))
        strip.save(out / f'{name}.webp', format='WEBP', quality=90, method=6)
        manifest['states'][name] = {
            'file': f'{name}.webp',
            'frames': len(imgs), 'fps': st['fps'], 'loop': st['loop'],
        }
    (out / 'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

    # preview: idle frame 0, tight square
    pv = states['idle']['frames'][0]
    bb = pv.getbbox()
    if bb:
        x0, y0, x1, y1 = bb; side = max(x1-x0, y1-y0) + 4
        cx, cy = (x0+x1)/2, (y0+y1)/2
        box = (int(cx-side/2), int(cy-side/2), int(cx+side/2), int(cy+side/2))
        sq = Image.new('RGBA', (side, side), (0, 0, 0, 0))
        sq.alpha_composite(pv.crop(box), (0, 0))
        sq.resize((256, 256), Image.Resampling.LANCZOS).save(out / 'preview.png')
    print('built', sid)

# ---------------- skins ----------------
SKINS = [
    dict(id='honey',    display='Honey',    hair_h=0.11,  hair_s=1.00, hair_v=1.00, dress_h=0.11, dress_s=0.90, dress_v=1.10, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='cream',    display='Cream',    hair_h=0.13,  hair_s=0.45, hair_v=1.00, dress_h=0.10, dress_s=0.55, dress_v=1.20, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='mint',     display='Mint',     hair_h=0.43,  hair_s=0.90, hair_v=1.00, dress_h=0.45, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.35),
    dict(id='peach',    display='Peach',    hair_h=0.05,  hair_s=1.00, hair_v=1.00, dress_h=0.04, dress_s=0.95, dress_v=1.10, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='night',    display='Night',    hair_h=0.65,  hair_s=1.00, hair_v=0.38, dress_h=0.65, dress_s=0.90, dress_v=0.75, skin_h=0.08, skin_s=1.05, skin_v=0.90, eye_h=0.75),
    dict(id='rose',     display='Rose',     hair_h=0.93,  hair_s=0.90, hair_v=1.00, dress_h=0.93, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='sky',      display='Sky',      hair_h=0.58,  hair_s=0.90, hair_v=1.00, dress_h=0.58, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='lavender', display='Lavender', hair_h=0.74,  hair_s=0.90, hair_v=1.00, dress_h=0.74, dress_s=0.90, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.75),
    dict(id='jade',     display='Jade',     hair_h=0.38,  hair_s=0.90, hair_v=0.95, dress_h=0.40, dress_s=0.90, dress_v=1.00, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.35),
    dict(id='coral',    display='Coral',    hair_h=0.995, hair_s=1.00, hair_v=1.00, dress_h=0.50, dress_s=1.00, dress_v=1.00, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='silver',   display='Silver',   hair_h=0.60,  hair_s=0.15, hair_v=0.98, dress_h=0.60, dress_s=0.35, dress_v=1.15, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='ocean',    display='Ocean',    hair_h=0.55,  hair_s=1.00, hair_v=0.55, dress_h=0.52, dress_s=1.00, dress_v=0.85, skin_h=0.08, skin_s=1.05, skin_v=0.90, eye_h=0.55),
    dict(id='berry',    display='Berry',    hair_h=0.88,  hair_s=0.95, hair_v=0.90, dress_h=0.88, dress_s=0.90, dress_v=1.00, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.75),
    dict(id='cocoa',    display='Cocoa',    hair_h=0.07,  hair_s=1.00, hair_v=0.55, dress_h=0.05, dress_s=0.90, dress_v=0.75, skin_h=0.06, skin_s=1.10, skin_v=0.75, eye_h=0.55),
    dict(id='snow',     display='Snow',     hair_h=0.55,  hair_s=0.12, hair_v=1.00, dress_h=0.55, dress_s=0.35, dress_v=1.20, skin_h=0.09, skin_s=1.00, skin_v=1.00, eye_h=0.55),
    dict(id='sunny',    display='Sunny',    hair_h=0.13,  hair_s=1.00, hair_v=1.00, dress_h=0.08, dress_s=1.00, dress_v=1.10, skin_h=0.08, skin_s=1.05, skin_v=0.92, eye_h=0.55),
]

def main():
    sources = {'idle': load('Idle'), 'walk': load('Walk'), 'jump': load('Jump'), 'dead': load('Dead')}
    print({k: len(v) for k, v in sources.items()})
    only = os.environ.get('ONLY')
    for skin in SKINS:
        if only and skin['id'] != only:
            continue
        build_skin(skin, sources)

if __name__ == '__main__':
    main()