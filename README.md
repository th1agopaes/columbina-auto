# Columbina Auto

> A top-down open-world driving game in Rust, on its own small 2D engine: **Sandbina**.

[![CI](https://github.com/th1agopaes/columbina-auto/actions/workflows/ci.yml/badge.svg)](https://github.com/th1agopaes/columbina-auto/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)
[![Rust 2024](https://img.shields.io/badge/rust-2024%20edition-orange?logo=rust)](https://doc.rust-lang.org/edition-guide/rust-2024/)
![Status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-yellow)

Walk the streets of a small tile city, jump into any car, dodge pedestrians and traffic, and keep your wanted level down while you finish short missions.

Inspired by the top-down classics *Grand Theft Auto* (1997) and *GTA 2* (1999). Not affiliated with Rockstar Games or Take-Two Interactive.

## 🧩 Sandbina, the engine

**Sandbina** (Sandrone + Columbina, two masks from the *commedia dell'arte*, and a nod to *sandbox*) draws every pixel on the CPU into a low-resolution buffer that winit and softbuffer show in a window. It follows one rule: **the engine only gains what the game needs**. The reasons for the stack are in [ADR 0001](docs/adr/0001-use-winit-and-softbuffer.md).

## 🗺️ Roadmap

Each phase ends with an 🔬 **X-ray**: a short look at the machine code of something we have just written, always in our own binaries.

| # | Phase | The engine gains | The game gains | 🔬 X-ray | Status |
| --- | --- | --- | --- | --- | --- |
| 0 | Foundation | Workspace, lints, check script, CI, ADR 0001 | — | `dev` × `release`: binary size and `strip` | ✅ |
| 1 | Window and loop | Window, scaled low-res buffer, fixed-timestep loop, keyboard | A square that moves | Clearing the screen in `dev` and `release` | ⏳ |
| 2 | Drawing | PNG loading, alpha blending, spritesheets, animation, bitmap text | An animated character on foot | Bounds checks: index × iterator | ⏳ |
| 3 | World | Tile map, following camera, AABB collision | A small city to walk around | How a `match` becomes machine code | ⏳ |
| 4 | Cars | Vectors and angles, rotated sprites | Arcade car physics; getting in and out | `f32` math and SIMD | ⏳ |
| 5 | Living city | Entities, state-machine AI, entity collision, sound | Pedestrians and traffic | `dyn` × generics: the vtable in the binary | ⏳ |
| 6 | Wanted and missions | A* pathfinding on the grid, HUD | Police, wanted level, 2–3 missions | Where the frame time goes | ⏳ |
| 7 | Release | Release build, icon | A release on itch.io | The executable in Ghidra | ⏳ |

## 🚀 Getting started

You need [Rust](https://www.rust-lang.org/tools/install) (stable, installed with `rustup`).

```sh
git clone https://github.com/th1agopaes/columbina-auto.git
cd columbina-auto
cargo run
```

For now the game only prints its title:

```text
Columbina Auto v0.1.0 (Sandbina v0.1.0)
```

## 🛠️ Development

```sh
./scripts/check.sh   # the green button: fmt, Clippy, tests and docs, the same checks as the CI
```

The CI runs the same script on Linux and Windows for every push and pull request. Branches, commit messages and the PR flow are in [CONTRIBUTING.md](CONTRIBUTING.md); changes are listed in [CHANGELOG.md](CHANGELOG.md).

```text
columbina-auto/
├── .github/              CI, Dependabot, issue and PR templates
├── crates/
│   ├── sandbina/         the engine (library)
│   └── columbina-auto/   the game (binary)
├── docs/
│   ├── adr/              architecture decision records
│   └── x-ray/            machine-code notes, one per phase
├── scripts/check.sh      the green button
└── Cargo.toml            workspace: shared version, lints and dependencies
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
