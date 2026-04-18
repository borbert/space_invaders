# Space Invaders — Polyglot Edition

A from-scratch Space Invaders clone implemented three times, in **Go**, **Rust**, and **Zig**. Same game, same feel, three very different languages — built to compare how each handles game loops, memory, and structure.

This is a learning project. The goal wasn't to ship a game; it was to feel the ergonomics of each language by building the same non-trivial thing in all three.

## Why three languages?

I've been working through some modern systems-language landscape — this repo targets Go, Rust, and Zig specifically. Writing the same game three times forces you past hello-world and into the real tradeoffs: how you structure state, where allocations happen, how errors propagate, how the build system feels when you actually depend on something.

Prior comparison work: I did the [1 Billion Row Challenge](https://medium.com/@borbert) in the same three languages. This is the game-dev companion piece.

## The implementations

| Language | Directory | Notes |
|----------|-----------|-------|
| Go       | [`go-invaders`](./go-invaders)     | [raylib-go](https://github.com/gen2brain/raylib-go); monolithic `main`, fixed arrays for bullets, invaders, and shields; 60 FPS loop. |
| Rust     | [`rust-invaders`](./rust-invaders) | [raylib](https://crates.io/crates/raylib) crate; split into `state`, `entities`, `collision`, and `config`; `GameState` drives updates. |
| Zig      | [`zig-invaders`](./zig-invaders)   | [raylib-zig](https://github.com/raylib-zig/raylib-zig) via `build.zig` / `build.zig.zon`; game logic and structs live in `main.zig`. |

Each folder has its own README with build and run instructions.

All three use **Raylib** for rendering and input, so behavior is intentionally close — with a few input differences called out below.

## Controls

### Common

- **Space** — fire (see per-folder notes for cooldown / hold behavior).
- **Enter** — after **Game Over** or **You Won**, restart the run.
- **Close the window** (or **Esc**, depending on your environment) — quit.

### Per implementation

**Go** and **Rust** — move with **Left / Right** arrows, **A / D**, or **H / L** (vim-style). **Space** shoots; Go uses an 8-frame cooldown and accepts key-down; Rust treats Space as pressed or held.

**Zig** — move with **Left / Right** only. **Space** uses a single press per shot (`isKeyPressed`), not hold-to-autofire like the other two.

## What I learned

A few honest observations after finishing all three:

- **Go** got me to a playable prototype fastest. Tooling is frictionless, and raylib-go plus a straight-line `main` loop was enough to stay productive without reaching for extra concurrency.
- **Rust** took the longest to get running but the least time to debug once it compiled. The borrow checker is annoying about mutable game state until you accept that `&mut self` on your world struct is fine, actually.
- **Zig** felt like writing C with better ergonomics and `comptime` superpowers. Explicit allocators made me think about lifetimes in a way that Go hides and Rust enforces — a nice middle ground.

## Repo layout