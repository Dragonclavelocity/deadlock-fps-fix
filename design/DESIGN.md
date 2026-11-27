# Deadlock FPS Fix — UI design spec

Source of truth for the Deadlock FPS Fix desktop UI and every image in this repo.
Screenshots, GIFs, the banner and the social preview are **rendered from the HTML in this folder**: edit the HTML, re-render, commit.
The approved main screen is `../_redesign/v2/deadlock-fps-fix.html` in the kit; its tokens and layout are copied here 1:1 and adapted from 1240×780 to 1280×800.

```
design/
├── dlf.css                # tokens + shared components
├── app.html               # full app UI, every screen; #screen=<id>, GIF demos with &demo=<id>
├── img/                   # official Deadlock Steam screenshots (see "Image sources"; replace before publishing)
├── marketing/
│   ├── gen_banner.py      # writes assets/banner.svg + marketing/banner.html
│   ├── banner-bg.jpg      # 1920×600 crop of img/deadlock-bg.jpg, embedded in the SVG as a data URI
│   ├── banner.html        # generated; renders assets/banner.png with the webfonts
│   └── social.html        # 1280×640 social preview
├── render.json            # render jobs (6 PNG, 3 GIF, banner, social)
└── render.py              # Playwright + ffmpeg renderer (copy of the kit's render.py)
```

Render (needs `pip install playwright pillow`, `playwright install chromium`, `ffmpeg` on PATH), from the repo root:

```bash
python design/marketing/gen_banner.py   # only after editing the banner
python design/render.py .               # all jobs
python design/render.py . gif-fov       # one job
```

There is no `design/anim/` folder: the three GIFs are driven by `app.html#screen=…&demo=…`.

## 1. Direction: cinematic, monochrome

The game's own art fills the window; the app sits on top of it like titles on a film still. One serif for headlines, one sans for everything else, one cream accent, no color.

| Do | Don't |
|---|---|
| Full-window screenshot under one flat shade `rgba(8,9,12,.5)` | Color fades or ramps, vignettes, blur, glow, colored shadows |
| Translucent flat cards `rgba(12,12,14,.8)` with a 1 px `rgba(255,255,255,.1)` edge | Frosted glass (`backdrop-filter`), drop shadows |
| Instrument Serif for headlines and big numbers; italic cream for the second line | Serif in body text, buttons or labels |
| Cream accent for the selected tab, the recommended card, the primary button, slider fill, toggles | A second accent; team colors; the game's logo or fonts |
| A different official screenshot behind each tab | Characters cut out, game logos, Valve branding as UI |
| Plain words, real numbers, "estimated" where it is an estimate | "VAC-safe", "ban-proof", "undetected", hype |

## 2. Color

| Token | Value | Use |
|---|---|---|
| `--stage` | `#0a0a0c` | behind the image |
| `--shade` | `rgba(8,9,12,.5)` | flat overlay over every screenshot |
| `--glass` | `rgba(12,12,14,.8)` | cards, tabs, pills, icon buttons |
| `--glass-on` | `rgba(16,15,13,.9)` | selected card |
| `--edge` | `rgba(255,255,255,.1)` | 1 px borders, hairlines |
| `--well` / `--chip` / `--track` | `rgba(0,0,0,.35)` / `rgba(255,255,255,.12)` / `rgba(255,255,255,.14)` | segmented control well / selected segment / slider and toggle track |
| `--text` / `--text-2` / `--text-3` | `#f1ece2` / `#b9b3a8` / `#7d776d` | primary / secondary / labels |
| `--acc` | `#ebe2cf` | the only accent (cream) |
| `--acc-ink` | `#15130f` | text and icons on the accent |

README badges: `labelColor=0c0c0e`, `color=ebe2cf` for release and the download button, `color=3a3833` for neutral badges.

## 3. Type

**Instrument Serif** (400, regular + italic) for headlines and big numbers; **Instrument Sans** (400–700) for UI; Consolas / Cascadia Mono only for file names, convars and launch options.

| Style | Spec |
|---|---|
| Screen headline | Serif 72 / 0.95, −0.01em; second line italic in `--acc` |
| Preset fps | Serif 64 / 1 (+ Sans 14 `--text-2` suffix) |
| Mini-card value | Serif 34 / 1 |
| Resolution card value | Serif 40 / 1 |
| Card title | Sans 15 / 600 |
| Body, list | Sans 13–14 / 400–500 |
| Eyebrow, captions | Sans 12.5–13, `--text-2` / `--text-3` |
| Mono | 12.5 |

Tabular numbers on (`font-feature-settings: "tnum"`).

## 4. Layout and components

- Window 1280 × 800 (rendered 2×). Padding 0 36 px 28 px. Title bar 56 px: ring mark, app name, two round icon buttons (launch options, settings), status pill, Windows 11 caption buttons on the right.
- Tabs in a glass capsule under the bar: FPS presets / Field of view / Resolution / Backups; the selected tab is a cream fill. Launch options and Settings are opened from the round icon buttons (cream fill when open).
- Every screen: **upper row** = eyebrow + two-line serif headline on the left, a 360 px side column with one or two mini cards on the right; **lower row** pinned to the bottom (`margin-top:auto`) with the working cards; **foot** line with a note and "Not affiliated with Valve".
- Radii: cards 18, mini cards 16, tabs 12 (segments 9), buttons 12 (small 10), photo frames 12, inputs 8. Buttons 44 px (resolution cards 40).
- Components: preset card (`.ps`), mini card with serif value (`.mini`), slider with a mark at 90° (`.track` + `u`), segmented control (`.seg`), photo frame (`.frame`), checkbox (`.ck`), toggle (`.tg`), tag (solid cream or ghost), key/value rows, comparison table, numbered steps, result line for launch options.

## 5. Screens

| `#screen=` | Background | Purpose | Key elements | Primary action |
|---|---|---|---|---|
| `presets` | `deadlock-bg.jpg` | Main screen (approved reference) | Hardware eyebrow, "Smooth frames, *even at 40 minutes.*", FOV and resolution mini cards, three preset cards with estimated fps | Apply Balanced |
| `fov` | `ss1.jpg` | Field of view | Slider 70–130 with the in-game max marked at 90, side-by-side preview (90° vs yours, crops of `ss4.jpg`, dashed 90° frame), "How it is applied" (in-game value vs `r_aspectratio` in `gameinfo.gi`), re-apply after updates | Apply 110° |
| `resolution` | `ss3.jpg` | Resolution | Current resolution mini card with segmented Native / 16:9 / 4:3 and custom W×H, three cards with a preview of `ss1.jpg` (4:3 card is the centre 75 % stretched to 16:9) and estimated fps | Applied / Stretch to 4:3 |
| `launch` | `ss1.jpg` zoomed on the fireplace | Launch options + shader cache | Options with checkboxes and plain explanations, result line, where to paste in Steam, what Clear removes | Copy launch options |
| `backups` | `ss2.jpg` zoomed on the right half | Restore | History list, "now / in backup" table, original settings card | Restore this backup |
| `settings` | `ss4.jpg` zoomed on the dome | Detection and options | Game and PC paths, behaviour toggles, "what this app changes / never does", language, updates | none (auto-saved) |

## 6. Numbers (keep consistent everywhere)

Test PC: RX 6700 XT 12 GB · Ryzen 5 5600 · 2560×1440 at 165 Hz · late-game team fights. Before: **94 fps**.
Presets (estimated, at 1920×1080 render resolution): Competitive **168 (+74)**, Balanced **141 (+47)**, Quality **112 (+18)**.
Frame caps: 237 / 162 / 162. Resolution with Balanced: native 2560×1440 **108**, 16:9 1920×1080 **141**, 4:3 1440×1080 **156**.
Field of view: **110°, was 90°**; `r_aspectratio ≈ tan(fov/2) / tan(45°) × 2.15` → 110° ≈ **3.07** (community mapping, see Verify).
Launch options: `-novid -w 1920 -h 1080 +fps_max 162`. Shader cache 1.4 GB. 5 backups, 184 KB, original settings from Nov 3.

## 7. Motion (GIFs)

All motion is deterministic through `window.__seek(t, dur)`; loops are 6 s at 12 fps, 960 px wide, and seamless (state at 6 s = state at 0 s). An arrow cursor shows clicks and drags. Only opacity and value changes, no slides.

| GIF | Source | Story |
|---|---|---|
| `deadlock-fps-presets.gif` | `app.html#screen=presets&demo=apply` | Click Apply Balanced → "Saving a backup" → "Clearing shader cache · 1.4 GB" → "Writing video.txt" → "Writing launch options" → "Applied · restart Deadlock" → fade back |
| `deadlock-fov-slider.gif` | `app.html#screen=fov&demo=fov` | Grab the knob at 110° → drag to 125° → down to 90° (experimental tag and dashed frame disappear) → back to 110°; preview, `r_aspectratio` and button update live |
| `deadlock-stretched-resolution.gif` | `app.html#screen=resolution&demo=res` | Click Stretch to 4:3 → card, tag, mini card and segmented control switch to 1440×1080 → click Use 16:9 → back |

A fourth demo (`#screen=backups&demo=restore`) exists in `app.html` but is not rendered.

## 8. Assets

| Asset | Source | Size |
|---|---|---|
| `assets/logo.svg` | hand-written: cream ring + bar on a dark rounded square (the title-bar mark) | 256×256 |
| `assets/banner.svg` | `marketing/gen_banner.py`: embedded photo (data URI), slow 12 s zoom, highlight moving between presets; system serif/sans stacks | 1280×400, ~200 KB |
| `assets/banner.png` | `marketing/banner.html` at t = 1 s | 2560×800 |
| `assets/social-preview.png` | `marketing/social.html` | 1280×640 |
| `assets/screenshots/*.png` | `app.html#screen=…` | 2560×1600 |
| `assets/gifs/*.gif` | `app.html#…&demo=…` | 960×600, 12 fps, 6 s |

Screenshot names: `deadlock-fps-presets`, `deadlock-fov-changer`, `deadlock-stretched-resolution`, `deadlock-launch-options`, `deadlock-settings-backup-restore`, `deadlock-fps-fix-settings`.

## 9. Image sources

Every background and preview photo in this repo is an **official Deadlock screenshot published by Valve on the Steam store page** (app 1422450, https://store.steampowered.com/app/1422450/Deadlock/). They are Valve's promotional images, © Valve Corporation. Using them in a design mockup is fine; **before publishing the repo, replace them with your own in-game captures** (or images you have rights to) and re-render. Do not use them as a logo or as if Valve endorsed the app.

| File | Where it is used | Source URL (Steam CDN) |
|---|---|---|
| `design/img/ss1.jpg` | FOV and launch screen backgrounds, resolution card previews | https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1422450/fa25560cf85debe669f5d4acd76cf81b0943019b/ss_fa25560cf85debe669f5d4acd76cf81b0943019b.1920x1080.jpg |
| `design/img/ss2.jpg` | Backups background (hero art, roster screen) | https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1422450/1a5a68e51d6aff5e2910410f84854c4c39af68cc/ss_1a5a68e51d6aff5e2910410f84854c4c39af68cc.1920x1080.jpg |
| `design/img/ss3.jpg` | Resolution background | https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1422450/52159118573367c304c43fc9ed85588c0c518a19/ss_52159118573367c304c43fc9ed85588c0c518a19.1920x1080.jpg |
| `design/img/ss4.jpg` | Settings background, FOV preview frames | https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1422450/ac02851ede518092df55746dbc98a199c91d7411/ss_ac02851ede518092df55746dbc98a199c91d7411.1920x1080.jpg |
| `design/img/deadlock-bg.jpg` | Presets background, banner (`marketing/banner-bg.jpg`, embedded in `assets/banner.svg` and `banner.png`), social preview | re-encoded copy of `ss4.jpg` (same URL as above) |

Matched by file size against the Steam store API (`appdetails?appids=1422450`) on 2026-11-27. The screenshots, GIFs, banner and social preview all contain these images, so all of them must be re-rendered after the swap. Fonts: Instrument Serif and Instrument Sans (SIL Open Font License, Google Fonts). Icons are hand-drawn SVG.

## 10. Verify before publishing

These are from community sources and forum posts, not Valve documentation, and Deadlock changes often. Check each one on the current build and adjust the app, READMEs, llms.txt and FAQ.

1. **In-game FOV cap.** The Camera FOV slider tops out at 90° (Deadlock forum thread "Increase FOV option to 120", March 2026; ProSettings recommends 90). Check the current range and the minimum.
2. **Where the in-game Camera FOV value is stored** (which file or convar the app writes for ≤ 90°). One guide names `citadel_camera_fov`; not confirmed.
3. **`r_aspectratio` in `game\citadel\gameinfo.gi`** (ConVars block) widening the view above 90°, and that it does not block matchmaking (forum posts from March 2026 say it still works). Also check whether Valve has since restricted it.
4. **FOV mapping** `r_aspectratio ≈ tan(fov/2)/tan(45°) × 2.15` (fitted to community values 1.75 ≈ 80°, 2.15 ≈ 90°, 2.49 ≈ 100°; a forum post uses 3.0). Approximate.
5. **`gameinfo.gi` is restored by game updates and by Steam "Verify integrity of game files"** (expected for a game file; check).
6. **`video.txt` path and keys:** `steamapps\common\Deadlock\game\citadel\cfg\video.txt` with `setting.defaultres` / `setting.defaultresheight` (forum reports). The names of the other keys the presets write (upscaling, shadows, AO, frame cap) still need to be read from a real file.
7. **Upscaling option names** in the current video settings ("FSR Performance / FSR Quality" in the mockup; guides also list a "Stretch" upscaling mode). Check the exact options and labels.
8. **Frame cap:** `+fps_max` as a launch option, and whether the in-game Maximum FPS setting is stored in `video.txt`.
9. **Launch options:** `-novid` (skips the intro video), `-w` / `-h`, `-high`, `-vulkan` (Vulkan renderer). `-vulkan` and `-dx11` were not confirmed by an official source; ProtonDB reports mention `-dx11`.
10. **Stretched resolution needs Fullscreen** (exclusive) display mode, and the HUD edge clipping at 4:3 reported on the Deadlock forums ("some UI elements not visible on 4:3 aspect ratio").
11. **Driver menu names:** AMD Software "GPU Scaling" + "Scaling Mode: Full panel"; NVIDIA Control Panel "Adjust desktop size and position → Full-screen, Perform scaling on: GPU".
12. **Shader cache locations:** `steamapps\shadercache\1422450`, `%LOCALAPPDATA%\AMD\DxCache`, `%LOCALAPPDATA%\NVIDIA\DXCache`, and that clearing them is safe while Steam is closed.
13. **Anti-cheat:** Deadlock uses VAC (check the Steam page's anti-cheat note).
14. **All FPS numbers** are illustrative for the mockup; replace them with real measurements before publishing.
15. **"Other ways to do it" table:** written at the time of writing from general knowledge; re-check before publishing.
