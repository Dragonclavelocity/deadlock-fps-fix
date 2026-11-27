# Security policy

## Supported versions

| Version | Supported |
|---|---|
| 1.0.x | Yes |
| < 1.0 | No |

## Reporting a vulnerability

Please **do not open a public issue**. Report privately through [GitHub Security Advisories](https://github.com/Dragonclavelocity/deadlock-fps-fix/security/advisories/new).

Include the version, Windows version, steps to reproduce and the impact. You will get an answer within 7 days, and a fix or a plan within 30 days for confirmed issues. We credit reporters in the release notes unless you prefer otherwise.

## Scope

- The app writes only to Deadlock's settings files, the shader cache folders, Windows settings and its own folder (`%APPDATA%\DeadlockFPSFix`). Anything that lets it write elsewhere is in scope.
- Update checks use GitHub Releases over HTTPS; installers are signed and come with `SHA256SUMS.txt`.
- Download Deadlock FPS Fix only from [GitHub Releases](https://github.com/Dragonclavelocity/deadlock-fps-fix/releases). Copies from other sites are not ours.
