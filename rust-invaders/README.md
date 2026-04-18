# Rust Invaders

Same Space Invaders–style game as the other folders, using the [`raylib`](https://crates.io/crates/raylib) crate. Part of the polyglot repo; see the [root README](../README.md) for context, controls overview, and comparison notes.

## Requirements

- **Rust** — a toolchain that supports **`edition = "2024"`** in [`Cargo.toml`](Cargo.toml) (see `rustc --version` / rustup if `cargo` errors on the edition).
- **Raylib** — the `raylib` crate links against the native library. If the build fails with missing `raylib` / `pkg-config` errors, install your platform’s Raylib development package so the linker can resolve it.

## Run

From the repository root:

```bash
cd rust-invaders
cargo run
```

## Tests

```bash
cd rust-invaders
cargo test
```

## Layout

- `src/main.rs` — window setup, input, main loop.
- `src/state.rs` — `GameState`, modes, update logic.
- `src/entities.rs` — entity types.
- `src/collision.rs` — hit tests (includes unit tests).
- `src/config.rs` — dimensions and tuning constants.

## Controls

Move with **Left / Right**, **A / D**, or **H / L**. **Space** counts as pressed or held for shooting. **Enter** restarts after game over or win. Zig-only differences: [root README — Controls](../README.md#controls).
