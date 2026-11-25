# Security policy

## Supported versions

Only the latest release and the `main` branch get fixes.

## Reporting a vulnerability

Please **do not open a public issue**. Report it privately through [GitHub's private vulnerability reporting](https://github.com/th1agopaes/columbina-auto/security/advisories/new), with the steps to reproduce and the version (window title or commit hash). You will get an answer as soon as possible, usually within a week.

## Scope

Columbina Auto is an offline, single-player game. The most relevant reports are:

- crashes or memory issues caused by malformed asset files (images, maps, sounds);
- vulnerable dependencies (also watched by Dependabot).

The workspace forbids `unsafe` code in our own crates (`unsafe_code = "forbid"` in `Cargo.toml`).
