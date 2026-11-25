# 1. Use winit and softbuffer for the window and software rendering

- **Status:** Accepted
- **Date:** 2026-11-25
- **Decider:** Thiago Paes (th1agopaes)

## Context

Columbina Auto is a top-down open-world driving game in the spirit of the late-1990s classics (GTA 1 and 2). It is also a learning project: its engine, Sandbina, is written from scratch so that drawing, the game loop, input and collision are understood, not borrowed. The engine only gains what the game needs.

The game needs:

- a window and keyboard input on **Windows** (where most players are) and **Linux** (development happens in WSL2 on Windows 11);
- a way to show an image drawn by our own code;
- a low internal resolution (e.g. 320×180, scaled up), which keeps the look of the originals and the work per frame small.

Versions checked on crates.io on 2026-11-25:

| Crate | Latest stable | Notes |
| --- | --- | --- |
| [winit](https://crates.io/crates/winit) | 0.30.13 | 0.31.0-beta.4 since 2026-10-24 (API changes) · Apache-2.0 |
| [softbuffer](https://crates.io/crates/softbuffer) | 0.4.8 | MIT OR Apache-2.0 |

## Decision

Use **winit 0.30** for the window, the event loop and the keyboard, and **softbuffer 0.4** to show a CPU-filled buffer of `u32` pixels in that window. Sandbina draws everything itself into the buffer: tiles, sprites, text, rotation and the camera.

- Drawing sits behind a `Renderer` trait, so a GPU backend (wgpu) can replace the CPU one later without touching the game.
- Both crates enter in Phase 1, the first time the game needs a window, as `[workspace.dependencies]`.
- Stay on the stable 0.30 line. Move to winit 0.31 in a dedicated PR once it is stable, and record it in a new ADR that supersedes this one.

## Alternatives considered

| Option | What it is | Why not (for now) |
| --- | --- | --- |
| [macroquad](https://crates.io/crates/macroquad) | Ready-made game library inspired by raylib | The game runs on day one, also on the web and on phones, but the engine would be macroquad, not ours |
| winit + [wgpu](https://wgpu.rs/) | Real GPU pipeline with WGSL shaders | Teaches modern graphics, but needs a lot of code before the first sprite, and wgpu ships new major versions several times a year |
| [SDL3](https://www.libsdl.org/) (`sdl3` crate) | Industry-standard C library: window, input, audio and 2D drawing | Needs the C library installed, and the Rust bindings are still 0.x |
| [minifb](https://crates.io/crates/minifb) | Window, pixel buffer and keyboard in a single crate | The simplest option, but it manages windows itself, and softbuffer's documentation considers that approach problematic; knowing winit carries over to wgpu later |
| [pixels](https://crates.io/crates/pixels) | Pixel buffer shown through wgpu, with GPU post-processing | Pulls in all of wgpu; softbuffer's documentation recommends softbuffer when GPU post-processing is not needed |

## Consequences

### Good

- We write every pixel, which is where most of the learning is.
- Two small crates from the same organization ([rust-windowing](https://github.com/rust-windowing)), with licenses compatible with ours (MIT OR Apache-2.0).
- No GPU needed: softbuffer also works where hardware acceleration does not (virtual machines, old computers, broken drivers).

### Bad

- CPU drawing is slow in the unoptimized `dev` profile. We will raise `opt-level` for `dev` and measure it in the Phase 1 X-ray.
- Image loading, sprites, text and rotation are ours to write.
- The winit 0.31 migration is coming (see Decision).

### Risk

- In WSL2, windows go through WSLg. The first window opens in Phase 1; if WSLg gets in the way, the fallback is to build and run the Windows target natively. The CI already builds on Windows.

## References

- [winit documentation](https://docs.rs/winit/0.30.13/winit/) · [softbuffer documentation](https://docs.rs/softbuffer/0.4.8/softbuffer/)
- [Fix Your Timestep!](https://gafferongames.com/post/fix_your_timestep/) (game loop) · [Game Programming Patterns](https://gameprogrammingpatterns.com/)
- [Are We Game Yet?](https://arewegameyet.rs/) (the Rust game ecosystem)
