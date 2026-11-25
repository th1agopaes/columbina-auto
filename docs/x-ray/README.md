# 🔬 X-ray

Each roadmap phase ends with a short look at the machine code (or the binary) of something we have just written, always in our own binaries. The goal is to see what the compiler really produces, not to guess.

| Phase | Question | Note |
| --- | --- | --- |
| 0 | How big is the binary in `dev` and `release`, and what does `strip` remove? | [phase-0-build-profiles.md](phase-0-build-profiles.md) |

Tools along the way: `file`, `nm`, `readelf`, [cargo-show-asm](https://github.com/pacak/cargo-show-asm) (`cargo asm`), [Compiler Explorer](https://godbolt.org/) and, in Phase 7, [Ghidra](https://ghidra-sre.org/).
