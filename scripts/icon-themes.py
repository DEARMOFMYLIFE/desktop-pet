"""Generate Yuzu's app icon themes (original flat art, MIT).

Themes: clock (default), star, moon, coffee, sun, leaf, heart, cloud, book, flower.
Outputs:
- backend/icons/themes/<theme>.png (1024)
- frontend/public/brand/icon-<theme>.png (128)
- backend/icons/icon.png + sizes/ico/icns/tray from the clock theme
"""
from __future__ import annotations
import io, math, struct
from PIL import Image, ImageDraw, ImageFilter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ICON = ROOT / 'backend' / 'icons'
THEMES = ICON / 'themes'
BRAND = ROOT / 'frontend' / 'public' / 'brand'
S = 2
W = 1024 * S
PAD = 28 * S
OUTLINE = (58, 46, 40, 255)
INK = (58, 46, 40, 255)
WHITE = (255, 253, 249, 255)

THEME_BG = {
    'clock': (255, 244, 230, 255),
    'star': (232, 224, 250, 255),
    'moon': (46, 58, 92, 255),
    'coffee': (255, 227, 214, 255),
    'sun': (255, 242, 204, 255),
    'leaf': (224, 242, 225, 255),
    'heart': (255, 224, 231, 255),
    'cloud': (222, 238, 250, 255),
    'book': (235, 228, 250, 255),
    'flower': (255, 236, 224, 255),
}

def badge(bg):
    im = Image.new('RGBA', (W, W), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.ellipse((PAD, PAD, W - PAD, W - PAD), fill=bg, outline=OUTLINE, width=round(9 * S))
    d.ellipse((PAD + 20 * S, PAD + 20 * S, W - PAD - 20 * S, W - PAD - 20 * S),
              outline=(255, 255, 255, 45), width=round(3 * S))
    return im

def star_points(cx, cy, ro, ri):
    pts = []
    for i in range(10):
        a = -math.pi / 2 + i * math.pi / 5
        r = ro if i % 2 == 0 else ri
        pts.append((cx + math.cos(a) * r, cy + math.sin(a) * r))
    return pts

def sparkle(d, x, y, r, fill):
    d.polygon([(x, y - r), (x + r * 0.32, y - r * 0.32), (x + r, y),
               (x + r * 0.32, y + r * 0.32), (x, y + r),
               (x - r * 0.32, y + r * 0.32), (x - r, y),
               (x - r * 0.32, y - r * 0.32)], fill=fill)

def face(d, cx, cy, color=INK, smile=True, sleepy=False):
    d.ellipse((cx - 62 * S, cy - 18 * S, cx - 30 * S, cy + 14 * S), fill=color)
    d.ellipse((cx + 30 * S, cy - 18 * S, cx + 62 * S, cy + 14 * S), fill=color)
    if sleepy:
        d.arc((cx - 62 * S, cy - 26 * S, cx - 30 * S, cy + 8 * S), 200 * S / 2, 340 * S / 2,
              fill=color, width=round(7 * S))
        d.arc((cx + 30 * S, cy - 26 * S, cx + 62 * S, cy + 8 * S), 200 * S / 2, 340 * S / 2,
              fill=color, width=round(7 * S))
    if smile:
        d.arc((cx - 28 * S, cy + 16 * S, cx + 28 * S, cy + 50 * S), 20, 160,
              fill=color, width=round(7 * S))

def clock():
    im = badge(THEME_BG['clock'])
    d = ImageDraw.Draw(im)
    c = 512 * S
    # feet
    for x in (392, 572):
        d.rounded_rectangle((x * S, 820 * S, (x + 60) * S, 884 * S), radius=18 * S,
                            fill=(240, 138, 106, 255), outline=OUTLINE, width=round(6 * S))
    # top knob
    d.ellipse((470 * S, 126 * S, 554 * S, 210 * S), fill=(240, 138, 106, 255),
              outline=OUTLINE, width=round(6 * S))
    # outer ring
    d.ellipse((182 * S, 182 * S, 842 * S, 842 * S), fill=(240, 138, 106, 255),
              outline=OUTLINE, width=round(8 * S))
    d.ellipse((222 * S, 222 * S, 802 * S, 802 * S), fill=WHITE,
              outline=OUTLINE, width=round(7 * S))
    # hands
    d.line((c, c, c, 340 * S), fill=INK, width=round(14 * S))
    d.line((c, c, 650 * S, 610 * S), fill=INK, width=round(14 * S))
    d.ellipse((c - 20 * S, c - 20 * S, c + 20 * S, c + 20 * S), fill=INK)
    # face
    face(d, c, 620 * S)
    return im

def star():
    im = badge(THEME_BG['star'])
    d = ImageDraw.Draw(im)
    c = (512 * S, 520 * S)
    d.polygon(star_points(c[0], c[1], 322 * S, 146 * S), fill=(255, 212, 106, 255),
              outline=OUTLINE)
    # thick outline via drawing the polygon twice with a wider line
    d.line(star_points(c[0], c[1], 322 * S, 146 * S) + [star_points(c[0], c[1], 322 * S, 146 * S)[0]],
           fill=OUTLINE, width=round(14 * S), joint='curve')
    face(d, c[0], 560 * S)
    for x, y, r in [(220, 270, 22), (800, 300, 16), (760, 760, 20)]:
        sparkle(d, x * S, y * S, r * S, (255, 255, 255, 235))
    return im

def moon():
    im = badge(THEME_BG['moon'])
    d = ImageDraw.Draw(im)
    # crescent: yellow circle, then badge-colour cut-out with outline
    d.ellipse((200 * S, 210 * S, 800 * S, 810 * S), fill=(255, 232, 176, 255),
              outline=OUTLINE, width=round(8 * S))
    d.ellipse((360 * S, 150 * S, 920 * S, 720 * S), fill=THEME_BG['moon'],
              outline=OUTLINE, width=round(8 * S))
    # sleepy face on the crescent
    face(d, 430 * S, 540 * S, sleepy=True)
    for x, y, r in [(760, 260, 18), (810, 760, 14)]:
        sparkle(d, x * S, y * S, r * S, (255, 232, 176, 245))
    return im

def coffee():
    im = badge(THEME_BG['coffee'])
    d = ImageDraw.Draw(im)
    # saucer
    d.ellipse((250 * S, 748 * S, 774 * S, 844 * S), fill=(247, 194, 174, 255),
              outline=OUTLINE, width=round(7 * S))
    # handle
    d.arc((600 * S, 430 * S, 820 * S, 690 * S), -80 * S / 2, 80 * S / 2, fill=OUTLINE,
          width=round(22 * S))
    # cup body
    d.rounded_rectangle((300 * S, 390 * S, 680 * S, 750 * S), radius=54 * S,
                        fill=WHITE, outline=OUTLINE, width=round(8 * S))
    # coffee
    d.ellipse((330 * S, 398 * S, 650 * S, 486 * S), fill=(197, 139, 99, 255),
              outline=OUTLINE, width=round(6 * S))
    # face on the cup
    face(d, 490 * S, 600 * S)
    # steam heart
    d.arc((430 * S, 245 * S, 510 * S, 330 * S), 160 * S / 2, 330 * S / 2, fill=OUTLINE,
          width=round(10 * S))
    d.arc((500 * S, 245 * S, 580 * S, 330 * S), 210 * S / 2, 380 * S / 2, fill=OUTLINE,
          width=round(10 * S))
    d.polygon([(505 * S, 322 * S), (455 * S, 400 * S), (555 * S, 400 * S)], fill=(240, 138, 106, 255))
    return im


def bezier(p0, p1, p2, p3, count=54):
    points = []
    for index in range(count + 1):
        t = index / count
        u = 1 - t
        points.append((
            u * u * u * p0[0] + 3 * u * u * t * p1[0] + 3 * u * t * t * p2[0] + t * t * t * p3[0],
            u * u * u * p0[1] + 3 * u * u * t * p1[1] + 3 * u * t * t * p2[1] + t * t * t * p3[1],
        ))
    return points


def sun():
    im = badge(THEME_BG['sun'])
    d = ImageDraw.Draw(im)
    c = 512 * S
    rays = []
    for index in range(12):
        angle = 2 * math.pi * index / 12
        start = (c + math.cos(angle) * 268 * S, c + math.sin(angle) * 268 * S)
        end = (c + math.cos(angle) * 366 * S, c + math.sin(angle) * 366 * S)
        rays.append((start, end))
    for start, end in rays:
        d.line((start, end), fill=OUTLINE, width=round(34 * S))
    for start, end in rays:
        d.line((start, end), fill=(250, 181, 74, 255), width=round(23 * S))
    d.ellipse((262 * S, 262 * S, 762 * S, 762 * S), fill=(255, 207, 95, 255),
              outline=OUTLINE, width=round(9 * S))
    d.arc((310 * S, 310 * S, 714 * S, 714 * S), 205, 320, fill=(255, 228, 157, 255),
          width=round(10 * S))
    face(d, c, 566 * S)
    for x, y in [(370, 650), (654, 650)]:
        d.ellipse((x * S, y * S, (x + 64) * S, (y + 30) * S), fill=(247, 165, 139, 95))
    return im


def leaf():
    im = badge(THEME_BG['leaf'])
    d = ImageDraw.Draw(im)
    # stem
    d.line(((512 * S, 744 * S), (512 * S, 892 * S)), fill=OUTLINE, width=round(28 * S))
    d.line(((512 * S, 744 * S), (512 * S, 892 * S)), fill=(93, 155, 103, 255), width=round(16 * S))
    left = bezier((512, 170), (292, 320), (320, 650), (512, 790))
    right = bezier((512, 790), (704, 650), (732, 320), (512, 170))
    outline = [(x * S, y * S) for x, y in left + right[1:-1]]
    d.polygon(outline, fill=(111, 174, 120, 255), outline=OUTLINE)
    d.line(outline + [outline[0]], fill=OUTLINE, width=round(9 * S), joint='curve')
    # central vein and side veins
    d.line(((512 * S, 218 * S), (512 * S, 754 * S)), fill=OUTLINE, width=round(7 * S))
    for y, spread in [(350, 92), (440, 130), (535, 132), (630, 98)]:
        d.line(((512 * S, y * S), ((512 - spread) * S, (y + 76) * S)), fill=OUTLINE,
               width=round(4 * S))
        d.line(((512 * S, y * S), ((512 + spread) * S, (y + 76) * S)), fill=OUTLINE,
               width=round(4 * S))
    face(d, 512 * S, 505 * S)
    sparkle(d, 760 * S, 300 * S, 24 * S, (255, 255, 255, 230))
    return im


def heart_points(cx, cy, scale=18):
    points = []
    for index in range(120):
        t = 2 * math.pi * index / 120
        x = 16 * math.sin(t) ** 3
        y = 13 * math.cos(t) - 5 * math.cos(2 * t) - 2 * math.cos(3 * t) - math.cos(4 * t)
        points.append((cx + x * scale, cy - y * scale))
    return points


def heart():
    im = badge(THEME_BG['heart'])
    d = ImageDraw.Draw(im)
    points = [(x * S, y * S) for x, y in heart_points(512, 458, 18)]
    d.polygon(points, fill=(238, 119, 131, 255), outline=OUTLINE)
    d.line(points + [points[0]], fill=OUTLINE, width=round(9 * S), joint='curve')
    # subtle highlight
    d.arc((330 * S, 350 * S, 690 * S, 680 * S), 200, 300, fill=(255, 176, 166, 255),
          width=round(12 * S))
    face(d, 512 * S, 535 * S)
    for x, y, r in [(220, 310, 24), (804, 340, 18), (744, 760, 20)]:
        sparkle(d, x * S, y * S, r * S, (255, 255, 255, 230))
    return im


def cloud():
    im = badge(THEME_BG['cloud'])
    d = ImageDraw.Draw(im)
    shapes = [
        ('ellipse', (370, 355, 660, 645)),
        ('ellipse', (302, 470, 586, 714)),
        ('ellipse', (548, 470, 832, 714)),
        ('rounded', (294, 542, 742, 760, 120)),
    ]

    def draw_shapes(expand, fill):
        for shape in shapes:
            if shape[0] == 'ellipse':
                x1, y1, x2, y2 = shape[1]
                d.ellipse(((x1 - expand) * S, (y1 - expand) * S, (x2 + expand) * S, (y2 + expand) * S),
                          fill=fill)
            else:
                x1, y1, x2, y2, radius = shape[1]
                d.rounded_rectangle(((x1 - expand) * S, (y1 - expand) * S, (x2 + expand) * S,
                                     (y2 + expand) * S), radius=(radius + expand) * S, fill=fill)

    draw_shapes(11, OUTLINE)
    draw_shapes(0, WHITE)
    face(d, 512 * S, 566 * S)
    for x, y, r in [(250, 300, 18), (800, 280, 22), (810, 760, 14)]:
        sparkle(d, x * S, y * S, r * S, (255, 255, 255, 240))
    return im


def book():
    im = badge(THEME_BG['book'])
    d = ImageDraw.Draw(im)
    # page block
    d.rounded_rectangle((320 * S, 268 * S, 776 * S, 786 * S), radius=34 * S,
                        fill=(255, 249, 234, 255), outline=OUTLINE, width=round(7 * S))
    # cover
    d.rounded_rectangle((272 * S, 242 * S, 740 * S, 802 * S), radius=38 * S,
                        fill=(105, 149, 176, 255), outline=OUTLINE, width=round(9 * S))
    # spine
    d.rounded_rectangle((272 * S, 242 * S, 372 * S, 802 * S), radius=34 * S,
                        fill=(81, 122, 149, 255), outline=OUTLINE, width=round(7 * S))
    d.line(((372 * S, 260 * S), (372 * S, 784 * S)), fill=OUTLINE, width=round(5 * S))
    # bookmark
    d.polygon([(530 * S, 242 * S), (620 * S, 242 * S), (620 * S, 414 * S), (575 * S, 374 * S),
               (530 * S, 414 * S)], fill=(240, 146, 116, 255), outline=OUTLINE)
    d.line([(530 * S, 242 * S), (620 * S, 242 * S), (620 * S, 414 * S), (575 * S, 374 * S),
            (530 * S, 414 * S), (530 * S, 242 * S)], fill=OUTLINE, width=round(5 * S))
    face(d, 552 * S, 574 * S)
    sparkle(d, 790 * S, 300 * S, 17 * S, (255, 255, 255, 230))
    return im


def flower():
    im = badge(THEME_BG['flower'])
    d = ImageDraw.Draw(im)
    # stem and leaves
    d.line(((512 * S, 620 * S), (512 * S, 892 * S)), fill=OUTLINE, width=round(28 * S))
    d.line(((512 * S, 620 * S), (512 * S, 892 * S)), fill=(98, 158, 108, 255), width=round(16 * S))
    for side in (-1, 1):
        d.ellipse(((512 + side * 44 - 84) * S, (748 - 30) * S, (512 + side * 44 + 84) * S,
                   (748 + 30) * S), fill=(122, 181, 128, 255), outline=OUTLINE, width=round(5 * S))
    # petals
    for index in range(6):
        angle = 2 * math.pi * index / 6 - math.pi / 2
        cx = 512 + math.cos(angle) * 176
        cy = 478 + math.sin(angle) * 176
        d.ellipse(((cx - 122) * S, (cy - 122) * S, (cx + 122) * S, (cy + 122) * S),
                  fill=(255, 190, 194, 255), outline=OUTLINE, width=round(7 * S))
    d.ellipse((354 * S, 320 * S, 670 * S, 636 * S), fill=(255, 216, 108, 255),
              outline=OUTLINE, width=round(8 * S))
    face(d, 512 * S, 506 * S)
    sparkle(d, 222 * S, 300 * S, 20 * S, (255, 255, 255, 230))
    return im

DRAW = {
    'clock': clock, 'star': star, 'moon': moon, 'coffee': coffee,
    'sun': sun, 'leaf': leaf, 'heart': heart, 'cloud': cloud, 'book': book, 'flower': flower,
}

def save_icns(images, path):
    chunks = []
    for typ, img in images.items():
        buf = io.BytesIO(); img.save(buf, format='PNG'); data = buf.getvalue()
        chunks.append(typ.encode('ascii') + struct.pack('>I', len(data) + 8) + data)
    body = b''.join(chunks)
    path.write_bytes(b'icns' + struct.pack('>I', len(body) + 8) + body)

def main():
    THEMES.mkdir(parents=True, exist_ok=True)
    BRAND.mkdir(parents=True, exist_ok=True)
    icons = {}
    for name, fn in DRAW.items():
        im = fn().resize((1024, 1024), Image.Resampling.LANCZOS)
        im.save(THEMES / f'{name}.png', format='PNG')
        im.resize((128, 128), Image.Resampling.LANCZOS).save(BRAND / f'icon-{name}.png')
        icons[name] = im
        print('theme', name)
    clock_icon = icons['clock']
    clock_icon.save(ICON / 'icon.png')
    clock_icon.resize((512, 512), Image.Resampling.LANCZOS).save(ICON / '512x512.png')
    clock_icon.resize((256, 256), Image.Resampling.LANCZOS).save(ICON / '128x128@2x.png')
    clock_icon.resize((256, 256), Image.Resampling.LANCZOS).save(ICON / 'icon_256.png')
    clock_icon.resize((128, 128), Image.Resampling.LANCZOS).save(ICON / '128x128.png')
    clock_icon.resize((32, 32), Image.Resampling.LANCZOS).save(ICON / '32x32.png')
    clock_icon.save(ICON / 'icon.ico', format='ICO',
                    sizes=[(256, 256), (128, 128), (64, 64), (48, 48), (32, 32), (16, 16)])
    save_icns({
        'ic07': clock_icon.resize((128, 128), Image.Resampling.LANCZOS),
        'ic08': clock_icon.resize((256, 256), Image.Resampling.LANCZOS),
        'ic09': clock_icon.resize((512, 512), Image.Resampling.LANCZOS),
        'ic10': clock_icon.resize((1024, 1024), Image.Resampling.LANCZOS),
        'ic11': clock_icon.resize((32, 32), Image.Resampling.LANCZOS),
        'ic12': clock_icon.resize((64, 64), Image.Resampling.LANCZOS),
        'ic13': clock_icon.resize((256, 256), Image.Resampling.LANCZOS),
        'ic14': clock_icon.resize((512, 512), Image.Resampling.LANCZOS),
    }, ICON / 'icon.icns')
    tray = clock_icon.resize((64, 64), Image.Resampling.LANCZOS)
    tray.save(ICON / 'tray.png')
    tray.save(ICON / 'tray-template.png')
    print('defaults + tray saved')

if __name__ == '__main__':
    main()