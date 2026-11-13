# Cute Girl character pipeline

Assets are derived from **Cute Girl - Free Sprites** by **pzUH** (CC0 1.0):

- https://opengameart.org/content/cute-girl-free-sprites
- https://www.gameart2d.com/cute-girl-free-sprites.html

## Requirements

- Python 3.11+
- Pillow
- NumPy

## Usage

```bash
# 1. Download CuteGirlFiles.zip from the OpenGameArt page and unzip:
#    scripts/cutegirl/source/png/*.png
python3 scripts/cutegirl/generate.py   # 16 palette skins -> frontend/public/assets/character/
python3 scripts/icon-themes.py         # app icon themes + tray + brand logos
```

`generate.py` recolors the five native clips (Idle / Walk / Jump / Dead) with
per-skin HSV palettes, builds the custom Sit / Focus / Sleep / Droop poses,
composes 200×200 frames and writes lossy WebP sprite sheets + manifests.

`source/` and `__pycache__/` are ignored.
