# Contributing to Deadlock FPS Fix

Thanks for helping. Bug reports, preset tuning, translations and docs fixes are all welcome.

## Ground rules

1. **Settings only.** A change may touch Deadlock's settings files (`video.txt`, and `gameinfo.gi` only for field of view), launch options, the shader cache folders or Windows options. Never the game process, its memory, game binaries or anti-cheat. Pull requests that inject, hook or read game memory are closed.
2. **Reversible.** Every new change must be covered by backup and restore, and must show the user what it changes before it runs.
3. **Honest numbers.** Preset estimates need a measurement: hardware, resolution, match situation and how you captured frame times (PresentMon, CapFrameX or similar). No "up to +200 FPS" claims.
4. **Plain words.** UI text is short, sentence case, no hype. Strings live in `locales/en.json` and `locales/es.json`; add both.

## Development

```bash
git clone https://github.com/Dragonclavelocity/deadlock-fps-fix
cd deadlock-fps-fix
npm install
npm run tauri dev
```

Needs Rust (stable), Node 20+ and the Tauri 2 prerequisites for Windows.

## Design and screenshots

The UI spec is in [`DESIGN notes are inline`](DESIGN notes are inline). If you change the UI, update `design/app.html` and re-render the README images:

```bash
pip install playwright pillow && playwright install chromium   # ffmpeg must be on PATH
python design/marketing/gen_banner.py   # only if the banner changed
python design/render.py .
```

## Pull requests

- One topic per PR, linked to an issue (`Closes #123`).
- Add a line to `CHANGELOG.md` under **Unreleased**.
- Fill in the PR template checklist.

## Translations

Copy `locales/en.json` to your language code, translate, and open a PR. For the README, keep the same sections as `README.md`.

By contributing you agree that your contribution is licensed under the MIT license.
