# Bulwark - Claude Agent Guide

## What is this project?

Bulwark is a multiplayer reimagining of the classic arcade game **Rampart** (Atari, 1990), written in Rust. It features phased gameplay: build walls (Tetris-like piece placement), place cannons, then battle. Up to 6 players, networked multiplayer, and Bot support.

## First steps for any new Claude session

1. Read this file (you're doing that now).
2. Read `PROJECT.md` — the living project plan. It contains:
   - Full Rampart reference (game mechanics)
   - Glossary of capitalized terms (App, Dedicated, Client, Server, Room, Owner, Player, Bot, Animation)
   - Technology stack decisions
   - Architecture overview
   - Phased implementation plan with status markers (`[ ]` / `[~]` / `[x]`)
   - Decision log
3. Check the phase status markers in `PROJECT.md` to understand where we are.
4. Look at recent git history (`git log --oneline -20`) to see what was last worked on.
5. If the user asks to work on something, find the matching phase in `PROJECT.md` and follow the plan.

## Project structure

This is a Cargo workspace:

```
bulwark/
├── CLAUDE.md          # This file
├── PROJECT.md         # Living project plan (READ THIS)
├── PROMPT.md          # Original project brief (historical reference)
├── Cargo.toml         # Workspace root
├── crates/
│   ├── bulwark-core/  # Shared types, game logic, config, serialization
│   ├── bulwark-net/   # Networking protocol, Transport trait, Client/Server messages
│   ├── bulwark-app/   # Game client executable (Macroquad, kira, gilrs, egui)
│   └── bulwark-dedicated/  # Headless server executable
└── data/              # Assets (textures, sounds, music) - placeholder initially
```

## Key conventions

- **Rust formatting**: Always run `cargo fmt` before committing. Use standard rustfmt defaults.
- **Terminology**: Use the capitalized Glossary terms from PROJECT.md (Client, Server, Room, etc.) consistently in code, comments, and conversation.
- **Architecture**: Client-Server separation is non-negotiable, even for local play. No special-casing local vs networked play in game logic.
- **The Client-Server boundary is sacred**: The Client must NEVER directly access Server or Room state — no shared memory, no `Arc<Mutex<...>>`, no shortcuts. The Client builds its own local copy of game state entirely from messages received over the Transport. This is what makes local and networked play identical.
- **Phased development**: Don't jump ahead. Complete the current phase, verify it works, then move to the next.
- **Commits are always done by the human developer** — Claude must never commit. The developer verifies and commits between phases.
- **Living document**: When completing a phase, update PROJECT.md: mark it `[x]`.
- **When in doubt, make Rampart**: If unsure about a game design decision, match the original Rampart behavior.

## Build & run

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo run -p bulwark-app       # Run the App
cargo run -p bulwark-dedicated # Run the Dedicated server
cargo test                     # Run all tests
cargo fmt                      # Format all code
cargo clippy                   # Lint check
```

## Tech stack summary

- **Framework**: Macroquad (immediate-mode 2D rendering, keyboard/mouse input, windowing)
- **Gamepad**: `objc2-game-controller` on macOS, `gilrs` on Windows/Linux (platform abstraction in `bulwark-app/src/gamepad/`)
- **Audio**: kira (game audio with mixer, crossfading, tweening)
- **UI**: egui + egui-macroquad (menus, settings)
- **Networking**: tokio + quinn (QUIC)
- **Serialization**: serde + bincode (network), serde + toml (config)
- **Logging**: tracing

## Important design decisions

- The App contains an embedded Server for local play. No networking is used for local games — a `LocalTransport` (async channels) connects Client to embedded Server.
- The `Transport` trait abstracts local vs network communication. Game logic never knows which it's using.
- All game state is authoritative on the Server/Room side. Clients have a read-only view and send action requests.
- Piece sequences use seeded RNG so all players get identical pieces (fairness).
- Each App has a persistent Player ID (stored in user.toml) for reconnection support.
