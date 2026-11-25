# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Cargo workspace with the `sandbina` engine (library) and the `columbina-auto` game (binary), sharing version, edition, license and lints.
- The game prints its title with the game and engine versions.
- `scripts/check.sh`: formatting, Clippy (pedantic), tests and docs in one command.
- CI on Linux and Windows, and Dependabot for crates and GitHub Actions.
- Issue forms, a pull request template and the community files (README, CONTRIBUTING, SECURITY).
- ADR 0001: winit and softbuffer for the window and software rendering.
- Phase 0 X-ray: binary size in the `dev` and `release` profiles.

[Unreleased]: https://github.com/th1agopaes/columbina-auto/commits/main
