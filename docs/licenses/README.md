# Character asset licenses

This directory will hold the license records for character sprites, fonts and other
third-party visual assets.

Required fields per asset:

| Field | Example |
|---|---|
| Name | `Toon Character Pack` |
| Author | `Example Author` |
| Source | `https://example.com/...` |
| License | `CC0-1.0` |
| Modifications | `resized to 256x256, recolored outline` |
| Used in | `frontend/public/assets/character/<skin>` |

## Assets currently used

| Name | Author | Source | License | Modifications |
|---|---|---|---|---|
| Cute Girl - Free Sprites | pzUH | https://opengameart.org/content/cute-girl-free-sprites | CC0 1.0 | 16 palette variants, 11 action states, custom sit/focus/sleep/droop poses, 200×200 |
| Yuzu app icon themes | Yuzu contributors | `scripts/icon-themes.py` | MIT | Original clock / star / moon / coffee icons, tray variants and brand logos |

Rules:

- Prefer CC0 / MIT / Apache-2.0 / OFL / CC-BY 4.0 (with attribution); custom permissive terms must be recorded verbatim.
- No CC-BY-NC, no "free for personal use", no unclear licenses.
- Keep the original license text alongside the asset.
- Update `LICENSES.md` when adding an asset.
