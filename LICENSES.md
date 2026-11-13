# Third-party licenses / 第三方许可

本项目**源代码**使用 **MIT License**，完整文本见根目录 [`LICENSE`](LICENSE)。

本文件专门记录**第三方资产与依赖**的许可，不能删除：
角色素材、应用图标、字体、Rust/JS 依赖都需要在这里留下来源与许可记录。

## Character assets / 角色素材

### Cute Girl - Free Sprites

- Author: pzUH
- Source: https://opengameart.org/content/cute-girl-free-sprites
- License: CC0 1.0 Universal (public domain)
- 16 palette variants, each with 11 action sheets, 200×200 frames
- Modifications: hair / dress / skin / eye HSV recolors; custom sit, focus,
  sleep and droop poses; composed app icons and brand mascot
- Generation: `scripts/cutegirl/generate.py`, `scripts/cutegirl/icons.py`
- Details: `docs/13-角色与图标资源.md`

### App icon themes and brand logos

- Original flat artwork generated in-repo by `scripts/icon-themes.py`
- License: MIT (same as project code)
- Themes: clock (default), star, moon, coffee
- Outputs: `backend/icons/themes/`, `backend/icons/`, `frontend/public/brand/icon-*.png`

No CC-BY-NC or unclear-license assets are used.

## UI dependencies

- Tailwind CSS — MIT — https://github.com/tailwindlabs/tailwindcss
- Lucide icons — ISC — https://github.com/lucide-icons/lucide
- React / React DOM — MIT — https://github.com/facebook/react

## Dependencies

- JavaScript: see `package-lock.json`
- Rust: see `backend/Cargo.lock`
