# Cute Boy character pipeline

Assets are derived from **The Boy - Free Sprites** by **pzUH** (CC0 1.0):

- https://opengameart.org/content/the-boy-free-sprites
- https://www.gameart2d.com/the-boy---free-sprites.html

## Requirements

- Python 3.11+
- Pillow
- NumPy

## Usage

```bash
# 1. Download FlatBoy.zip from the OpenGameArt page and unzip:
#    scripts/cuteboy/source/png/*.png
python3 scripts/cuteboy/generate.py  # 16 palette skins -> frontend/public/assets/character/cuteboy/
```

`generate.py` recolors the five native clips (Idle / Walk / Jump / Run / Dead)
with the same per-skin HSV palettes used by the Cute Girl pack, builds the
custom Sit / Focus / Sleep / Droop poses, composes 200×200 frames and writes
WebP sprite sheets + manifests.

`source/` and `__pycache__/` are ignored.
