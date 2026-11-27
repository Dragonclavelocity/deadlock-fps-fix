# Changelog

All notable changes to Deadlock FPS Fix are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [1.0.0] - 2026-11-27

### Added
- FPS presets for Deadlock: Competitive, Balanced and Quality, with estimated fps for late-game fights on your hardware and a "Best for your PC" pick.
- Field of view: slider from 70° to 130° with a before and after preview. Up to 90° uses the in-game Camera FOV value; above 90° writes the community `r_aspectratio` value to `game\citadel\gameinfo.gi` (experimental) and can re-apply it after game updates.
- Resolution: native, lower 16:9 and 4:3 stretched presets plus custom sizes, written to `video.txt` and the `-w`/`-h` launch options, with a GPU scaling check for AMD and NVIDIA.
- Shader cache clear (Steam shader pre-cache and the DirectX cache).
- Launch options builder (`-novid`, `-w`/`-h`, `+fps_max`, optional `-vulkan`) with copy to clipboard.
- Backups before every change, history with a "now vs backup" table, one-click restore, and an "Original settings" backup kept forever.
- Settings: game and file detection, behaviour toggles, English and Spanish interface, update check from GitHub Releases, copy diagnostics.

[Unreleased]: https://github.com/Dragonclavelocity/deadlock-fps-fix/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Dragonclavelocity/deadlock-fps-fix/releases/tag/v1.0.0
