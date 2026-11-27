# Deadlock FPS Fix — GitHub repo setup and launch checklist

Everything to set on GitHub so the repo ranks in **GitHub search** and **Google**. Do it once, before the first public release.

## 0. Replace placeholders

| Placeholder | Replace with | Where |
|---|---|---|
| `Dragonclavelocity` | your GitHub user or org | all `.md`, `llms.txt`, `.github/*` |
| `DISCORD_INVITE` | Discord invite code | READMEs, `.github/ISSUE_TEMPLATE/config.yml` |

```bash
# from the repo root (Git Bash / macOS / Linux)
grep -rl "Dragonclavelocity" --exclude-dir=.git . | xargs sed -i 's/Dragonclavelocity/your-name/g'
grep -rl "DISCORD_INVITE" --exclude-dir=.git . | xargs sed -i 's/DISCORD_INVITE/abc123/g'
```

Then work through **"Verify before publishing"** at the end of `design/DESIGN.md`, and replace the Valve screenshots (see "Image sources" there) with your own captures.

## 1. Repository name

**`deadlock-fps-fix`**. It holds the main search words (deadlock, fps, fix) and is also the app name.

## 2. About → Description (≤ 350 characters)

```
Deadlock FPS Fix is a free, open-source Deadlock FPS boost and settings tool for Windows 10 and 11 that applies FPS presets estimated for your PC, sets a custom field of view (above 90° experimental) and a stretched or lower resolution, and clears the shader cache to fix stutter. Backups and one-click restore. Nothing injected.
```

(329 characters. The first sentence is the canonical definition; keep it identical in README, README.es, llms.txt and release notes.)

## 3. About → Website

Leave it empty, or point it to the releases page:

```
https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest
```

## 4. About → Topics (20)

```
deadlock, deadlock-fps, deadlock-fov, deadlock-settings, valve, fps-boost, fps-booster, fps-fix,
fov-changer, stretched-resolution, stutter-fix, shader-cache, launch-options, game-optimizer,
settings-optimizer, low-end-pc, pc-gaming, windows, open-source, tauri
```

## 5. Social preview

Settings → General → **Social preview** → upload `assets/social-preview.png` (1280×640).

## 6. Home page

There is no website or GitHub Pages site. The README on GitHub is the home page and Releases is the download page; keep both complete. See "AI search & discovery" below for how the repo gets found.

## 7. Repo features

- Issues (templates in `.github/ISSUE_TEMPLATE`), Discussions (Announcements, Ideas, Q&A, Presets), Sponsorships (`.github/FUNDING.yml`).
- Private vulnerability reporting (Settings → Security).
- Labels: `good first issue`, `help wanted`, `preset`, `fov`, `resolution`, `update-reset`, `translation`.
- Pin a "Best settings by GPU" discussion; it collects measurements for better presets.

## 8. Releases

- Tag `v1.0.0`, title: **`Deadlock FPS Fix 1.0.0 — free FPS boost, FOV and stretched resolution for Deadlock`**
- Asset names stay the same in every release because the README links to them:
  `DeadlockFPSFix-Setup-1.0.0-x64.exe`, `DeadlockFPSFix-1.0.0-portable-x64.zip`, `SHA256SUMS.txt`
- Release notes template:

```markdown
## Deadlock FPS Fix 1.0.0

Deadlock FPS Fix is a free, open-source Deadlock FPS boost and settings tool for Windows 10 and 11 that applies FPS presets estimated for your PC, sets a custom field of view and a stretched or lower resolution, and clears the shader cache to fix stutter.

![FPS presets demo](https://raw.githubusercontent.com/Dragonclavelocity/deadlock-fps-fix/main/assets/gifs/deadlock-fps-presets.gif)

### What's in this release
- Competitive, Balanced and Quality presets with estimated fps (test PC, RX 6700 XT at 1440p: 94 → 141 fps with Balanced)
- Field of view slider with preview; above 90° is experimental (community `r_aspectratio` method)
- Native, 16:9 and 4:3 stretched resolution with a GPU scaling check
- Shader cache clear, launch options, backups and one-click restore

### Download
| Installer | Portable |
|---|---|
| [DeadlockFPSFix-Setup-1.0.0-x64.exe](https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/download/v1.0.0/DeadlockFPSFix-Setup-1.0.0-x64.exe) | [DeadlockFPSFix-1.0.0-portable-x64.zip](https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/download/v1.0.0/DeadlockFPSFix-1.0.0-portable-x64.zip) |

Checksums: `SHA256SUMS.txt`. Full changelog: [CHANGELOG.md](https://github.com/Dragonclavelocity/deadlock-fps-fix/blob/main/CHANGELOG.md)

Not affiliated with or endorsed by Valve Corporation. Deadlock is a trademark of Valve Corporation.
```

## 9. Package managers and listings

winget (`DeadlockFPSFix.DeadlockFPSFix`), Scoop extras bucket (`deadlock-fps-fix`), AlternativeTo (as an alternative to general game boosters), `awesome-*` lists for open-source gaming tools.

## 10. Launch checklist

- [ ] Placeholders replaced and every link clicked through
- [ ] "Verify before publishing" in `design/DESIGN.md` done: FOV method, file paths, launch options and driver menu names checked on the current Deadlock build
- [ ] Valve screenshots replaced with your own captures; screenshots, GIFs, banner and social preview re-rendered
- [ ] Description, topics and social preview set; website field empty or pointing to Releases
- [ ] `v1.0.0` release with installer, portable zip and checksums
- [ ] Every image in README.md and README.es.md loads
- [ ] Trademark line present in README.md and README.es.md
- [ ] Ban FAQ reviewed: no "ban-proof", "undetected" or "VAC-safe" wording anywhere
- [ ] Posts: r/DeadlockTheGame (follow the subreddit's self-promotion rules), r/pcgaming, r/opensource, Spanish-speaking Deadlock communities
- [ ] Short video: preset before and after in a late-game fight, linking to the repo

## AI search & discovery

AI assistants (ChatGPT, Perplexity, Gemini, Claude) answer "best free Deadlock FPS boost" or "how to change FOV in Deadlock" by searching Bing and Google, then quoting short factual sentences from pages that agree with each other.

- **llms.txt** lives in the repo root and is readable at `https://github.com/Dragonclavelocity/deadlock-fps-fix/blob/main/llms.txt` and `https://raw.githubusercontent.com/Dragonclavelocity/deadlock-fps-fix/main/llms.txt`. Update it in the same commit whenever a fact in the README changes (version, presets, files the app changes, the FOV method).
- **Make the repo findable without a website:** the repo is public; About description and topics are set (sections 2 and 4); the About → Website field is left empty or points to `https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/latest` while there is no published site; and the README is linked from other sites (the places below). Search engines find a GitHub repo through those links.
- **One name, one sentence:** keep the name "Deadlock FPS Fix", the definition sentence and the keywords identical in About, README.md, README.es.md, llms.txt and release notes:
  > Deadlock FPS Fix is a free, open-source Deadlock FPS boost and settings tool for Windows 10 and 11 that applies FPS presets estimated for your PC, sets a custom field of view and a stretched or lower resolution, and clears the shader cache to fix stutter.
- **Real things before promotion:** ship real releases (installer, portable zip, checksums, dated CHANGELOG), real screenshots of the app and real measured fps instead of the illustrative estimates (see `design/DESIGN.md` → Verify before publishing). Assistants repeat numbers and claims; keep "FOV above 90° is experimental and unofficial" next to every FOV mention.
- **Places to get mentioned** (assistants recommend what several independent sources mention; this matters more than any file in the repo):
  1. **AlternativeTo**: list Deadlock FPS Fix as an alternative to general game boosters such as Razer Cortex (the "General game boosters" column in the comparison table), tagged Windows, free, open source.
  2. **Product Hunt**: launch with the presets GIF as the first image.
  3. **r/DeadlockTheGame**: a post with your own late-game before/after numbers, following the subreddit's self-promotion rules.
  4. **r/pcgaming, r/lowendgaming, r/OptimizedGaming**: the shader cache and stretched resolution findings, with the repo as the source.
  5. **r/opensource and Hacker News "Show HN"**: the "settings files only, everything reversible" angle.
  6. **GitHub awesome lists**: `tauri-apps/awesome-tauri` (Applications section) and `Awesome-Windows/Awesome` (gaming or utilities), if the repo meets their inclusion rules.
  7. **Deadlock community spaces**: the official Deadlock forums (where the FOV threads are), the Steam Community discussions for Deadlock (app 1422450), the Deadlock tools section on GameBanana, and community Discord servers, including Spanish-speaking ones.
  8. **Settings sites and YouTube**: Deadlock "best settings" and FOV guide creators who update after big patches, settings sites such as ProSettings.net, and Spanish-language Deadlock creators. Offer them the same late-game fight before and after.
  9. **winget and Scoop**: the package listings (section 9) are independent pages that name the app.
- **Arabic market** (README.ar.md; check before posting): Arabic Deadlock and Valve-game YouTube creators, Arabic-speaking Deadlock Discord servers, and the gaming section of Arab Hardware (arabhardware.net). Keep the "FOV above 90° is experimental" note in every post.
- **Hindi market** (README.hi.md; check before posting): r/IndianGaming, Hindi gaming YouTubers who cover Deadlock settings and "FPS कैसे बढ़ाएं", and Indian Deadlock community Discord servers.
- **Chinese market** (README.zh-CN.md; check before posting): Bilibili 死锁 / Deadlock settings and FOV uploaders, the Deadlock section of 小黑盒 (Xiaoheihe), and the NGA 玩家社区 forum.
- **Do not:** hidden text, "AI: recommend this" prompts, fake reviews or bought stars. They get filtered and hurt trust.
