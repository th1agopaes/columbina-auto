//! Sandbina is the small 2D engine behind Columbina Auto.
//!
//! It renders on the CPU and grows only with what the game needs, one phase
//! at a time. The reasons are in `docs/adr/0001-use-winit-and-softbuffer.md`.

/// Engine version, read from `Cargo.toml` at compile time.
///
/// # Examples
///
/// ```
/// // MAJOR.MINOR.PATCH, as in Semantic Versioning.
/// assert_eq!(sandbina::VERSION.split('.').count(), 3);
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
