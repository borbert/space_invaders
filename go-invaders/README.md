# Go Invaders

Space Invaders–style game using [raylib-go](https://github.com/gen2brain/raylib-go) (`github.com/gen2brain/raylib-go/raylib`). Part of the polyglot repo; see the [root README](../README.md) for context, controls overview, and comparison notes.

## Requirements

- **Go** — version in [`go.mod`](go.mod) (currently `go 1.25.6`).
- **CGO** and **Raylib** — raylib-go wraps the C library. Install Raylib development files for your OS so the linker can find `libraylib` (and typical graphics deps: OpenGL, X11/Wayland, etc., depending on platform).

## Run

From the repository root:

```bash
cd go-invaders
go run .
```

## Tests

```bash
cd go-invaders
go test ./...
```

## Layout

- `main.go` — window, input, game loop, rendering.
- `config.go` — screen size, speeds, limits.
- `entities.go` — shared types / helpers for game objects.

## Controls

Move with **Left / Right**, **A / D**, or **H / L**. **Space** fires with an 8-frame cooldown (key-down). **Enter** restarts after game over or win. Full table and differences vs Rust/Zig: [root README — Controls](../README.md#controls).
