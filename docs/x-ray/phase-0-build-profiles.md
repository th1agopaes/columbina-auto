# Phase 0 · `dev` × `release`: binary size and `strip`

- **Date:** 2026-11-25
- **Toolchain:** rustc 1.100.0 (stable), `x86_64-unknown-linux-gnu`, on WSL2 (Ubuntu 26.04)
- **Code:** branch `phase-0/foundation` at `e373844` (the game prints its title)

## Question

How big is the game binary in each [profile](https://doc.rust-lang.org/cargo/reference/profiles.html), what makes the difference, and what does `strip` remove?

## How

```sh
cargo build                                              # dev
cargo build --release                                    # release
cp target/release/columbina-auto /tmp/release-default
CARGO_PROFILE_RELEASE_STRIP=symbols cargo build --release # release, symbols stripped
stat -c '%s %n' target/debug/columbina-auto /tmp/release-default target/release/columbina-auto
file target/debug/columbina-auto /tmp/release-default target/release/columbina-auto
nm -C target/debug/columbina-auto | grep 'columbina_auto::'
nm -C /tmp/release-default | grep 'columbina_auto::'
nm target/release/columbina-auto
readelf -S -W target/debug/columbina-auto | grep -c debug
readelf -S -W /tmp/release-default | grep -c debug
```

`CARGO_PROFILE_<PROFILE>_<KEY>` overrides one profile setting for a single command, without editing `Cargo.toml` (see [Cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html#profile)).

## Results

| Build | Bytes | KiB | `file` says | `.debug_*` sections | Our functions in `nm -C` |
| --- | ---: | ---: | --- | ---: | --- |
| `dev` | 4,554,920 | 4,448 | `with debug_info, not stripped` | 8 | `main`, `title` |
| `release` | 447,872 | 437 | `not stripped` | 0 | `main` |
| `release` + `strip = "symbols"` | 339,160 | 331 | `stripped` | 0 | none (`no symbols`) |

## What we learned

- **`dev` is about 10× bigger than `release`.** Most of the gap is debug info (the 8 `.debug_*` sections that map machine code back to files, lines and variables), plus unoptimized code (`opt-level = 0`).
- **`release` has no debug info at all, not even the standard library's.** The standard library ships precompiled with debug info; since Rust 1.77, Cargo strips debug info by default when no crate asks for it ([Rust 1.77.1 announcement](https://blog.rust-lang.org/2024/03/28/Rust-1.77.1/)). That default is off for Windows MSVC targets, so a Windows build can look different.
- **Even `release` is ~437 KiB for a 10-line program.** The standard library (formatting, `println!`, panic handling) is linked into the binary; our own code is a tiny part of it.
- **`strip = "symbols"` saves another 108,712 bytes (−24%)** by removing the symbol table (function names and addresses). The binary still runs: the CPU uses addresses, never names.
- **Inlining:** in `release`, `title` is gone as a symbol. The optimizer copied its body into `main` and removed the call; `dev` keeps every function separate, which is why it is easier to debug.

## Decision

Keep the default `release` profile (symbols kept) while developing: the next X-rays and the Ghidra session in Phase 7 need function names, and so do readable backtraces. Strip symbols only in the build we distribute, in Phase 7. No change to `Cargo.toml` for now.
