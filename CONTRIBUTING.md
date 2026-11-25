# Contributing

Thanks for your interest! This is a learning project built in public, phase by phase (see the roadmap in the [README](README.md)). Bug reports, ideas and questions are welcome as [issues](https://github.com/th1agopaes/columbina-auto/issues/new/choose).

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install) (stable) with `rustup`.
2. Clone the repository and run `cargo run`.
3. Before every commit, run the green button:

   ```sh
   ./scripts/check.sh
   ```

   It checks formatting (`cargo fmt`), lints (`cargo clippy`, pedantic, warnings as errors), tests (`cargo test`) and docs (`cargo doc`). The CI runs the same script on Linux and Windows.

## Workflow

1. **Issue:** every phase or feature starts as an issue with acceptance criteria.
2. **Branch** from `main`: `phase-N/short-name` for roadmap phases (`phase-1/window-and-loop`), `feat/...`, `fix/...` or `docs/...` for the rest.
3. **Atomic commits:** one logical change per commit, each one passing `./scripts/check.sh`.
4. **Pull request** using the template, with `Closes #N`. The CI must be green.
5. **Squash merge**, with the PR title as the commit message, in Conventional Commits.

## Commit messages

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), in English, in the imperative ("add", not "added"), and signed.

```text
type(scope): short summary in lowercase

Optional body: what and why, wrapped at 72 columns.
```

| Type | For |
| --- | --- |
| `feat` | New behavior in the game or the engine |
| `fix` | Bug fix |
| `refactor` | Code change without new behavior |
| `perf` | Faster or smaller, same behavior |
| `test` | Tests only |
| `docs` | Documentation, ADRs, X-ray notes |
| `style` | Formatting only |
| `build` | Cargo manifests, profiles, dependencies |
| `ci` | CI, Dependabot, scripts used by the CI |
| `chore` | Anything else (licenses, templates, editor config) |

| Scope | Covers |
| --- | --- |
| `engine` | `crates/sandbina` |
| `game` | `crates/columbina-auto` |
| `ci` | `.github/workflows`, `scripts/` |
| `docs` | `docs/` and the Markdown files |
| `assets` | `assets/` |
| `deps` | Dependency updates (`build(deps)`, `ci(deps)`, also used by Dependabot) |

Changes that touch the whole repository go without a scope (`chore: add editorconfig`).

## Code style

- `rustfmt` decides formatting (100 columns, Rust 2024 style); Clippy pedantic decides the rest. Silence a lint only with `#[expect(lint, reason = "...")]`.
- Names, comments and docs in English. Comments explain **why**; the code already says what.
- Every public item gets a `///` doc comment (`missing_docs` is on).
- No `unsafe` in our crates (`unsafe_code = "forbid"`).
- Code is organized by subject (`render/`, `input.rs`, `car.rs`), never by kind (`utils/`, `helpers/`).

## Dependencies

- A dependency comes in only when the game needs it, declared once in `[workspace.dependencies]` and inherited with `name.workspace = true`.
- It enters in its own `build(deps)` commit. Choices that shape the architecture get an [ADR](docs/adr/README.md).
- Third-party assets are listed with their origin and license in `CREDITS.md`.

## Licensing

By contributing, you agree that your contributions are dual licensed under MIT OR Apache-2.0, like the rest of the project (see [License](README.md#license)).
