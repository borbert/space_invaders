# Zig Invaders

Space Invaders–style game using [raylib-zig](https://github.com/raylib-zig/raylib-zig), pulled in via `build.zig` / [`build.zig.zon`](build.zig.zon). Part of the polyglot repo; see the [root README](../README.md) for context, controls overview, and comparison notes.

## Requirements

- **Zig** — at least the version in [`build.zig.zon`](build.zig.zon) `minimum_zig_version` (currently **0.15.2**).
- **Network (first build)** — `zig build` may fetch the `raylib_zig` git dependency; after that, builds can be offline if the cache is populated.

## Run

From the repository root:

```bash
cd zig-invaders
zig build run
```

## Build only

Produces the executable under the default install prefix (typically `zig-out/bin/`):

```bash
cd zig-invaders
zig build
```

## Tests

```bash
cd zig-invaders
zig build test
```

## Layout

- `src/main.zig` — entry point and most game logic.
- `src/root.zig` — module root wired in `build.zig` for the `zig_invaders` module and tests.

## Controls

**Left / Right** only for movement. **Space** fires one shot per key press (`isKeyPressed`), not hold-to-autofire. **Enter** restarts after game over or win. Full control table: [root README — Controls](../README.md#controls).
