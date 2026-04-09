# Bulwark - Project Plan

> A multiplayer reimagining of the classic arcade game **Rampart** (Atari Games, 1990), built in Rust.
> This is a **living document**. Each phase is updated as it is implemented.

---

## Table of Contents

1. [Rampart Reference](#rampart-reference)
2. [Glossary](#glossary)
3. [Technology Stack](#technology-stack)
4. [Architecture Overview](#architecture-overview)
5. [Phase Plan](#phase-plan)
6. [Departures from Rampart](#departures-from-rampart)

---

## Rampart Reference

This section captures the mechanics of the original arcade Rampart so that any developer (human or AI) working on Bulwark has a shared understanding of the source material. **When in doubt, make Rampart.**

### Game Flow

The game begins with **Castle Selection** (~20 seconds, first round only) where each player picks a starting "home castle" with their cursor. Then the cycle begins:

1. **Cannon Placement Phase** (~10-15 seconds) - Place cannons inside your claimed territory
2. **Battle Phase** (~10-20 seconds) - Fire cannons at opponents' walls and cannons. Ends with "Cease Fire!" announcement; phase waits for all cannonballs in flight to land before transitioning.
3. **Build/Repair Phase** (~20-30 seconds) - Place Tetris-like wall pieces to surround castles and repair damage

After the build phase, territory evaluation occurs (flood-fill), then back to cannon placement. The first round grants 3 starting cannons before the first battle.

Between phases, a brief announcement/transition (~0.5-1 second) plays with a voice callout for the next phase.

### Build Phase Mechanics

- **Pieces**: Polyomino shapes ranging from **1-cell singles** up to **5-cell pentominoes**. Stored as 3x3 grids with ~10-12 base shapes; rotation provides additional variants. **Pieces get larger and more complex as the game progresses** (early rounds give simpler shapes). Pieces are presented one at a time; place one, get the next. Pieces are the same sequence for all players (fairness).
- **Placement**: Pieces float freely (they do NOT fall like Tetris). Move with cursor, rotate clockwise with Rotate button. A piece is placed with the Accept button. Pieces cannot overlap existing walls, castles, cannons, water, craters, grunts, or the map edge. **5 consecutive invalid placement attempts auto-ends the build phase** (anti-stall).
- **Territory claiming**: When the timer expires, a flood-fill algorithm runs from the map borders inward. Any area completely enclosed by an unbroken wall loop that contains at least one castle becomes that player's claimed territory. The territory visually fills with the player's color. Cannons outside enclosed walls become inactive/unusable. Foreign walls inside newly claimed territory are eliminated.
- **Elimination**: If a player cannot surround at least one castle at the end of the build phase, they are eliminated ("Game Over") and become a spectator.

### Cannon Placement Phase

- **First round**: 3 starting cannons.
- **Subsequent rounds**: Home castle (the one selected at game start) grants 2 additional cannons. Each other enclosed castle grants 1 additional cannon.
- Cannons must be placed within claimed/enclosed territory.
- **Cannons occupy a 2x2 grid space** (not 1x1).
- Cannot overlap walls, castles, other cannons, water, or craters.
- Phase ends early if all cannons are placed or timer expires.
- **Strategic note**: Cannons fire in **placement order**. Cannons closer to enemy territory mean shorter flight times and faster effective reload.

### Battle Phase

- Players aim a crosshair/cursor and press Fire to launch a cannonball at the cursor position.
- Cannonballs travel in a **parabolic arc** with fixed speed. Near targets arrive quickly (nearly straight trajectory); far targets arc high and take significantly longer. Players must lead their aim.
- **Each cannon can only have ONE cannonball in flight at a time.** With 5 cannons, at most 5 cannonballs can be airborne simultaneously. Cannons fire in **placement order** (first placed fires first).
- After a cannon's shot lands, that cannon can fire again immediately (no cooldown beyond flight time).
- On impact: **a single grid cell is destroyed** (the exact target cell). Wall cells become rubble; cannons become ruined (can no longer fire during this battle phase).
- **Clearing occurs just before each Build phase.** Wall rubble is cleared to buildable ground. Ruined cannons persist as unbuildable obstacles through **one Build phase**, then are cleared before the next. This means a destroyed cannon is a 1-round impediment to building.
- Phase ends with "Cease Fire!" and waits for all airborne cannonballs to land before transitioning.

### Grid and Territory

- The original uses a **~40x30 tile grid** with 8x8 pixel tiles (336x240 screen resolution).
- **2-player**: Map split by a river/harbor into halves. A "composite" mode also exists with ships.
- **3-player**: An **inverted Y-shaped river system** divides the map into three roughly equal sections.
- Each section contains 2-3 castles and varied terrain (ground, water, coastline).
- **Castles occupy 2x2 grid tiles.**
- The original has 6 hand-crafted single-player maps of increasing difficulty (more water, fewer castles).

### Visual Style

- Top-down perspective, medieval theme.
- Green grass terrain, blue water, grey/tan stone walls, castle structures.
- Player territories are color-coded (claimed territory fills with player color tint).
- Cannonballs are small dark projectiles with shadow/arc animation.
- Explosions are brief, bright flashes with debris.
- Craters are dark spots left on the ground after impacts.
- UI overlays show: timer, current phase name, piece preview, player scores.

### Audio

The arcade uses OKI6295 (sampled audio) and YM2413 (FM synthesis). Music by Brad Fuller and Don Diekneite. Key tracks:

- **Castle Selection**: "Select" theme (~45s)
- **Cannon Placement**: "Place Cannons" theme (~18s)
- **Battle start**: Digitized speech — "Ready... Aim... Fire!"
- **Battle end**: Digitized speech — "Cease Fire!"
- **Build/Repair phase**: "Surround A Castle" — the most iconic, hectic track (~48s)
- **Territory claimed**: "Castle Bonus" sting (~16s)
- **Elimination**: "Dismissal" theme (~33s)
- **Victory**: Distinct 1-player and 2-player victory themes
- **Fanfares**: Short 2-second stings between phases

**Sound effects**: Cannon boom, cannonball flight whistle, explosion impact, piece placement click, error buzz on invalid placement.

**Digitized speech** (British-accented male announcer): "Use ROTATE for a better fit!", "Surround a castle to keep playing", phase announcements.

### Scoring

- Points for: captured territory (per tile), captured castles (bonus), destroyed enemy ships/walls.
- Larger enclosed areas score more.
- Score is the **tiebreaker** in multiplayer when "Final Battle" ends with multiple survivors.
- Score display visible on-screen throughout.

### Original Controls

- **3-player dedicated cabinet (1990)**: Trackball per player + 2 buttons (Fire/Place, Rotate). Analog, fluid cursor movement.
- **2-player joystick conversion kit (1991)**: Standard JAMMA joystick + 2 buttons. Same PCB, different ROMs.
- Cursor wraps/clamps to playfield boundaries.

---

## Glossary

These terms are capitalized throughout all project documentation when referring to their specific architectural meaning.

| Term | Definition |
|------|-----------|
| **App** | The game client executable. Provides graphics, sound, and input for the Player. Contains an embedded Server for local play. |
| **Dedicated** | The headless server executable for remote multiplayer. Hosts multiple Rooms simultaneously. |
| **Client** | The component within the App that communicates with a Server (either embedded or remote). Manages the local view of game state, renders graphics, handles input. |
| **Server** | The authoritative game state manager. Exists embedded within the App (for local play) or within the Dedicated (for network play). Manages Rooms. |
| **Room** | A single game session managed by a Server. Contains all game state, Animations, Players, and Bots. The source of truth. |
| **Owner** | The Client that created a Room. Has special privileges (start game, add Bots, manage lobby). |
| **Player** | A human being playing the game. Represented in a Room via their Client connection. |
| **Bot** | A computer-controlled Player, managed entirely Server-side. Appears as a normal Player in the Room to other Clients. |
| **Animation** | A synchronized visual/audio event in the Room state. Has: unique ID, type enum (FIRE, CANNONBALL, EXPLOSION, etc.), start time (monotonic ms), duration, coordinates for interpolation. Broadcast to all Clients. Server manages lifecycle and cleanup. |

### Identity

Each App generates a unique ID (a long hex/SHA-like string) on first launch and stores it in the user settings file. This ID persists across sessions and is used for reconnection. A command-line override (`--player-id`) exists for debugging multiple Apps on one machine.

### Room Codes

When a Room is created on a Dedicated, it gets a random 4-letter code (e.g., "XKFR") that other Players use to join. Communicated out-of-band by the Players themselves.

---

## Technology Stack

### Language & Build

- **Rust** (stable toolchain), formatted with `rustfmt`
- **Cargo workspace** with shared crates:
  - `bulwark-core` - Game logic, state, types, serialization (shared by App and Dedicated)
  - `bulwark-net` - Networking protocol, Client/Server communication
  - `bulwark-app` - The App executable (graphics, audio, input, UI)
  - `bulwark-dedicated` - The Dedicated executable (headless server)

### Key Dependencies

| Purpose | Crate | Rationale |
|---------|-------|-----------|
| Game framework / rendering | `macroquad` | Lightweight immediate-mode 2D framework. Built-in windowing (miniquad), keyboard/mouse input, texture/sprite drawing. Minimal boilerplate, fast compiles, cross-platform. |
| In-game UI / menus | `egui` + `egui-macroquad` | Immediate-mode UI on top of Macroquad. Settings screens, lobby UI, debug overlays. In-game HUD drawn directly with Macroquad draw calls for custom look. |
| Audio | `kira` | Purpose-built game audio. Mixer graph, crossfading, clock-synced transitions, tweening. Pure Rust (uses `cpal` under the hood). |
| Gamepad input | `gilrs` | Gold standard for Rust gamepad support. Xbox/PS/Switch Pro, hot-plugging, mapped inputs. |
| Networking | `tokio` + `quinn` | Async runtime + QUIC protocol. Reliable streams for control messages, unreliable datagrams for frequent state snapshots. Built-in TLS 1.3 encryption. |
| Serialization (network) | `serde` + `bincode` | Compact binary serialization for network packets. |
| Serialization (config) | `serde` + `toml` | Human-readable config files. |
| Logging | `tracing` + `tracing-subscriber` | Structured logging, useful for both App and Dedicated. |
| RNG | `rand` | Piece generation, map generation, Room codes. Seeded RNG for deterministic piece sequences. |
| UUID/ID generation | `uuid` | Player IDs, or a simpler hex approach. |

### Build Outputs & Packaging

- `cargo build --release` produces two binaries: `bulwark-app` and `bulwark-dedicated`
- **Portable distribution**: A directory containing the binary, default `.toml` config files, and a `data/` directory for assets.
- **macOS**: Can be wrapped in a `.app` bundle. User preferences stored in `~/Library/Application Support/com.bulwark.app/` (only on macOS).
- **Windows/Linux**: User preferences stored as `user.toml` next to the executable.
- No registry usage, no APPDATA, no system-level dependencies beyond what Rust/Macroquad link.

---

## Architecture Overview

```
+----------------------------------------------------------+
|                        App                                |
|                                                           |
|  +------------+    +------------+    +-----------------+  |
|  |   Client   |--->|  Server    |    |    Macroquad    |  |
|  | (game view,|<---|  (embedded,|    | (render, audio, |  |
|  |  input,    |    |  local     |    |  input, UI)     |  |
|  |  actions)  |    |  Room)     |    +-----------------+  |
|  +-----+------+    +------------+                         |
|        |                                                  |
|        | (same interface, local or network)                |
+--------|-------------------------------------------------+
         |
    [Network boundary - only when using Dedicated]
         |
+--------|-------------------------------------------------+
|        v                                                  |
|  +-----------+                                            |
|  |  Server   |    Dedicated                               |
|  |           |    (headless, no graphics)                  |
|  | +-------+ |                                            |
|  | | Room 1| |    Manages multiple Rooms simultaneously   |
|  | +-------+ |                                            |
|  | | Room 2| |                                            |
|  | +-------+ |                                            |
|  | | Room 3| |                                            |
|  | +-------+ |                                            |
|  +-----------+                                            |
+----------------------------------------------------------+
```

### The Client-Server Boundary (Critical Rule)

**The Client must NEVER directly access Server or Room state.** Even when the Server is embedded in the same App process, the Client only knows about the game through messages received over the Transport. The Client maintains its own **local copy** of the game state, built entirely from `ServerMessage`s it receives. Rendering, audio, and UI are driven solely from this local copy.

This means:
- No shared memory, no `Arc<Mutex<RoomState>>`, no "just peek at the Room" shortcuts.
- The Client's local state may be slightly behind the Server's authoritative state (especially over the network). This is expected and correct.
- If the Client needs information, it must have been sent by the Server. If the Server hasn't sent it, the Client doesn't know it.
- This rule is what makes local and networked play identical — if you can't cheat the boundary locally, networking just works.

### Transport

The Client and Server communicate through a **trait-based interface** (`Transport`) that has two implementations:
1. **Local**: Async channels for embedded Server in App. Messages are still serialized/deserialized across the boundary (same as network) to enforce the separation and catch serialization bugs early.
2. **Network**: QUIC streams via `quinn` for Dedicated connections.

Both implementations present the same API to game logic, so there are no special cases between local and networked play.

### Message Types

- **Client -> Server**: `JoinRoom`, `CreateRoom`, `PlacePiece`, `PlaceCannon`, `FireCannon`, `MoveCursor`, `LeaveRoom`, `StartGame`, `AddBot`, `RemoveBot`
- **Server -> Client**: `RoomState` (full sync), `StateDelta` (incremental), `AnimationEvent`, `PhaseChange`, `Error`, `PlayerJoined`, `PlayerLeft`, `GameOver`, `Victory`

### Reconnection

If a Client disconnects, the Server keeps the Room alive for a configurable timeout (default: 60 seconds). The Client can reconnect using its persistent Player ID, and the Server sends a full `RoomState` to resync. If the timeout expires, the Player is removed from the Room. If the Owner disconnects and times out, ownership transfers to the next Player, or the Room is destroyed if empty.

---

## Phase Plan

Each phase below is designed to be small, verifiable, and committable. Phases are grouped into milestones. **The human developer handles all git commits** — Claude should never commit. After each phase, the developer will verify and commit before moving on.

Status markers:

- `[ ]` Not started
- `[~]` In progress
- `[x]` Complete

### Milestone 1: Project Skeleton

#### Phase 1.1: Cargo Workspace & Hello World `[x]`

**Goal**: Both executables compile and print to stdout.

- Initialize Cargo workspace with 4 crates: `bulwark-core`, `bulwark-net`, `bulwark-app`, `bulwark-dedicated`
- `bulwark-app` prints "Bulwark App v0.1.0" and exits
- `bulwark-dedicated` prints "Bulwark Dedicated Server v0.1.0" and exits
- `bulwark-core` exports a `VERSION` constant used by both
- Add `.gitignore`, `rustfmt.toml` (standard settings), basic `README.md`
- Verify: `cargo build` succeeds, both binaries run on the developer's OS

#### Phase 1.2: Configuration System `[x]`

**Goal**: Both executables read TOML config files.

- Define config structs in `bulwark-core`:
  - `AppConfig` - window settings (resolution, fullscreen mode, vsync), audio volume, server address for multiplayer
  - `DedicatedConfig` - listen address/port, max rooms, timeouts
  - `UserConfig` - keybindings, player name, player ID (auto-generated if absent)
- Load from TOML files with serde, with sensible defaults if files are missing
- App looks for `app.toml` and `user.toml` next to executable (macOS: also `~/Library/Application Support/com.bulwark.app/user.toml`)
- Dedicated looks for `dedicated.toml` next to executable
- Both print loaded config summary on startup
- Verify: Edit TOML values, see them reflected in startup output

#### Phase 1.3: Macroquad Window & Basic Rendering `[x]`

**Goal**: App opens a GPU-accelerated window with configurable graphics mode.

- Add `macroquad` dependency to `bulwark-app`
- Create a Macroquad app (`#[macroquad::main]`) that opens a window with title "Bulwark"
- Apply `AppConfig` window settings via `Conf`: resolution, fullscreen/windowed/borderless
- Render a colored background (the "ocean blue" base color)
- Draw a simple test grid of colored rectangles to verify 2D rendering works
- Verify: Window opens at configured resolution, mode switching works, grid renders

#### Phase 1.4: Input Foundation `[ ]`

**Goal**: Keyboard, mouse, and gamepad input is detected and displayed.

- Set up Macroquad keyboard/mouse input + gilrs for gamepad
- Display a debug overlay (text on screen) showing:
  - Last key pressed
  - Mouse position and button state
  - Gamepad connection status, last button/axis input
- Define an `InputAction` enum: `Up, Down, Left, Right, Accept, Cancel, RotateCW, RotateCCW`
- Map physical inputs to `InputAction` using the keybind config from `UserConfig`
- Verify: All three input types register on screen, remapping via TOML works

### Milestone 2: App State Machine & Menus

#### Phase 2.1: App State Machine `[ ]`

**Goal**: App has distinct states with transitions.

- Define `AppState` enum: `Splash, MainMenu, Settings, Lobby, InGame, Spectating`
- Implement a simple state machine (match on current state in main loop, each state has its own update/draw functions)
- `Splash` state: Show "BULWARK" title text for 2 seconds, auto-transition to `MainMenu`
- `MainMenu`: Placeholder menu items (text list)
- Transitions: Splash -> MainMenu (auto), MainMenu -> Settings / Lobby (on selection)
- Verify: App boots to splash, transitions to menu, selections change state

#### Phase 2.2: Menu System `[ ]`

**Goal**: Navigable menus driven entirely by keyboard/gamepad (mouse optional).

- Implement a reusable menu component: vertical list of items, highlight current selection, scroll if needed
- `MainMenu` items: "New Game (Local)", "New Game (Multiplayer)", "Join Game", "Settings", "Quit"
- `Settings` submenu: Graphics, Audio, Controls (placeholder sub-screens for now)
- Navigate with Up/Down/Accept/Cancel actions from any input device
- Menu selection sound effect (placeholder beep)
- Verify: Full menu navigation with keyboard and gamepad, Cancel returns to parent menu

#### Phase 2.3: Settings Screens `[ ]`

**Goal**: Configurable settings that persist to `user.toml`.

- **Graphics**: Resolution picker, display mode (Windowed/Fullscreen/Borderless), VSync toggle
- **Audio**: Master volume, Music volume, SFX volume (slider-style, adjusted with Left/Right)
- **Controls**: Show current bindings, allow rebinding (press key/button to bind)
- Changes apply immediately where possible (audio volume), or on confirm (resolution)
- Save to `user.toml` on confirm
- Verify: Change a setting, restart App, setting persists

### Milestone 3: Core Game Types & Local Server

#### Phase 3.1: Game State Types `[ ]`

**Goal**: Define all core game data types in `bulwark-core`.

- `Grid`: 2D tile grid. Each cell is an enum: `Ground, Water, Wall, Castle, Cannon, Crater`. Castles occupy 2x2 tiles, Cannons occupy 2x2 tiles.
- `GamePhase`: enum `Lobby, CastleSelection, CannonPlacement, Battle, Build, Evaluation, GameOver, Victory` (matches original Rampart order)
- `PlayerState`: id, name, color, territory cells, score, eliminated flag, cursor position, current piece
- `RoomState`: grid, phase, timer, list of players, list of Animations, round number, piece RNG seed
- `Animation`: id, type enum, start_time_ms, duration_ms, coordinates, associated player
- `Piece`: shape (2D bool array), rotation state
- Piece catalog: Define the standard polyomino shapes (I, O, T, S, Z, L, J, plus 3-cell and 5-cell variants)
- Serialize/deserialize all types with serde + bincode
- Unit tests for serialization round-trips
- Verify: `cargo test` passes, types serialize correctly

#### Phase 3.2: Transport Trait & Local Implementation `[ ]`

**Goal**: Define the Client-Server communication interface and implement the local (in-process) version.

- Define `ClientMessage` and `ServerMessage` enums in `bulwark-net`
- Define `Transport` trait with async send/receive for both directions
- Implement `LocalTransport`: pair of async channels (tokio mpsc), still serializing/deserializing messages to enforce the boundary
- Unit tests: send messages through LocalTransport, verify receipt
- Verify: `cargo test` passes

#### Phase 3.3: Server & Room Logic (Stub) `[ ]`

**Goal**: A Server that can create a Room, accept a Client, and manage lobby state.

- `Server` struct: manages a `HashMap<RoomCode, Room>`
- `Room` struct: holds `RoomState`, processes `ClientMessage`s, emits `ServerMessage`s
- Implement: `CreateRoom` -> assigns Owner, generates Room code; `JoinRoom` -> adds Player; `LeaveRoom` -> removes Player
- Room starts in `Lobby` phase. Owner can `StartGame` (transitions to first Build phase) and `AddBot`/`RemoveBot`
- Minimal game phase transitions (just the state changes, no gameplay logic yet)
- Verify: Unit tests exercise create/join/leave/start flow

#### Phase 3.4: Client Wiring `[ ]`

**Goal**: App's Client connects to embedded Server and exercises the lobby flow.

- When user selects "New Game (Local)" from MainMenu, App creates a `LocalTransport`, starts embedded Server, creates a Room
- App transitions to `Lobby` state, displays: Room info, Player list, "Start Game" / "Add Bot" buttons
- Owner can add/remove Bots (names shown in player list) and Start Game
- Starting game transitions Room to Build phase and App to InGame state (blank game screen for now)
- Verify: Full flow from MainMenu -> Lobby -> InGame, player list updates correctly

### Milestone 4: The Grid & Build Phase

#### Phase 4.1: Grid Rendering `[ ]`

**Goal**: Render the game grid on screen.

- Generate a simple test map: ground with water borders, a few castles placed
- Render each cell as a colored tile (green=ground, blue=water, grey=wall, etc.)
- Camera: fit the entire grid on screen with appropriate padding
- Grid should scale to window size while maintaining aspect ratio
- Verify: A static test grid renders correctly, looks clean at various window sizes

#### Phase 4.2: Map Data & Loading `[ ]`

**Goal**: Hand-crafted maps for each player count, loaded from data files.

- Define a map data format (TOML or simple grid text file) specifying: grid dimensions, tile types per cell, castle positions, region assignments (which cells belong to which player's starting region)
- Create 2 maps per player count (2-player through 6-player = 10 maps total). Grid scales larger for more players. Water boundaries divide regions with organic-looking coastlines.
- Each map ensures: roughly equal region sizes, 2-3 castles per region, sufficient buildable ground area
- Load maps from the `data/` directory at runtime
- Map selection: for now, pick randomly from the available maps for the player count. Future: let Owner choose in lobby.
- Store region ownership in grid metadata
- Verify: Load and render each map, visually inspect fairness across all player counts

#### Phase 4.3: Piece Generation, Cursor & Placement `[ ]`

**Goal**: Players can see and place wall pieces on the grid during build phase, with polished cursor and ghost piece rendering.

- Implement seeded RNG piece generator (all players get same sequence)
- **Cursor**: grid-snapped for keyboard/gamepad (one tap = one cell), nearest-cell snapping for mouse. Distinct visual cursor indicator.
- **Ghost piece**: semi-transparent overlay of current piece at cursor position. Tinted green when placement is valid, **tinted red when invalid** (overlapping walls, water, etc.)
- Move piece with LRUD input, rotate with RotateCW/RotateCCW
- Accept places the piece if valid (no overlap with existing objects, within player's region)
- Invalid placement: red ghost tint + error sound
- Next piece appears immediately after placement
- Show 1-2 upcoming pieces in a preview area
- Build phase timer displayed prominently
- Verify: Place pieces on the grid, ghost piece tints correctly, rotation works, invalid placements rejected, timer counts down

#### Phase 4.4: Territory Evaluation (Flood Fill) `[ ]`

**Goal**: After build phase, determine claimed territory.

- Implement flood-fill algorithm: starting from each castle, flood outward on ground tiles, stopping at walls and water. If the flood cannot reach the map edge (i.e., the castle is fully enclosed), the enclosed area is claimed.
- Alternative approach: flood from the outside inward, marking all reachable ground. Any unreachable ground containing a castle is claimed territory.
- Color claimed territory with player's color tint
- Check elimination: any player with zero enclosed castles is eliminated
- Animate the territory fill (brief visual sweep)
- Verify: Build walls around a castle, see territory claimed. Leave a gap, territory not claimed. Surround multiple castles, all claimed.

### Milestone 5: Cannon Placement & Battle

#### Phase 5.1: Cannon Placement Phase `[ ]`

**Goal**: Players place cannons in their territory.

- Calculate cannon count per player based on territory size (formula TBD, roughly: 1 + territory_cells / threshold)
- Display available cannon count
- Player moves cursor within their territory, Accept places a cannon on valid ground
- Cannons occupy one cell, cannot overlap walls/castles/other cannons
- Timer for placement phase
- Verify: Cannon count scales with territory, placement works, invalid spots rejected

#### Phase 5.2: Battle Phase - Aiming & Firing `[ ]`

**Goal**: Players can aim and fire cannons.

- Cursor becomes a crosshair that can move freely (grid-snapped or analog depending on input mode)
- Accept button fires the next available cannon (cycle through cannons with cooldown)
- Display which cannon is about to fire (highlight)
- Fire creates a `CANNONBALL` Animation with start position (cannon), end position (crosshair), and flight duration (based on distance)
- Battle phase timer
- Verify: Aiming cursor works, firing creates visible cannonball animation data

#### Phase 5.3: Cannonball Flight & Impact `[ ]`

**Goal**: Cannonballs visually fly and destroy things on impact.

- Render cannonball Animation: interpolate position along arc from start to end over duration
- On Animation completion, Server evaluates impact:
  - Destroy wall cells in a small radius (1-2 cells) around impact point
  - Destroy cannon if directly hit
  - Create crater cell at impact point
- Create `EXPLOSION` Animation at impact point
- Sound effects: cannon boom at fire, whistle during flight, explosion at impact
- Verify: Fire cannon, see cannonball fly, walls/cannons destroyed on impact, craters appear

#### Phase 5.4: Round Loop `[ ]`

**Goal**: Complete game loop cycling through all phases.

- After battle phase: re-evaluate territory (flood fill), check eliminations
- Transition: Battle -> Evaluation -> Build (if >1 player) or Victory
- Phase transition fanfare and announcements ("BUILD PHASE", "BATTLE!", etc.)
- Eliminated players see "GAME OVER" overlay, can spectate
- Last player standing sees "VICTORY" screen
- Return to lobby/menu after game end
- Verify: Play a complete multi-round game loop, elimination works, victory triggers

### Milestone 6: Bot AI

> Bots before networking — lets the solo developer play full games locally and validate all gameplay before adding network complexity.

#### Phase 6.1: Bot Framework `[ ]`

**Goal**: Server can run Bot players that take actions.

- Bot struct: has a player state, runs decision logic each game tick
- Bot receives the same game state view as a Client would
- Bot emits the same `ClientMessage` actions as a human Player
- Wire into Room: Bots tick alongside the normal game loop

#### Phase 6.2: Bot Build Phase AI `[ ]`

**Goal**: Bots can place wall pieces intelligently.

- Strategy: identify gaps in walls, attempt to place pieces to close them
- Prioritize enclosing castles that are close to being surrounded
- Handle piece rotation to find best fit
- Difficulty scaling: harder bots place faster and make better choices

#### Phase 6.3: Bot Battle Phase AI `[ ]`

**Goal**: Bots can aim and fire cannons at opponents.

- Strategy: target opponent walls near their castles (maximize damage to enclosures)
- Vary accuracy by difficulty level
- Fire rate management: don't waste all cannons immediately

### Milestone 7: Networking & Multiplayer

#### Phase 7.1: Network Transport `[ ]`

**Goal**: Implement the `Transport` trait over QUIC.

- Implement `NetworkTransport` using `quinn` (QUIC)
- Client connects to Server address, establishes bidirectional streams
- Serialize `ClientMessage`/`ServerMessage` with bincode over QUIC
- Handle connection lifecycle: connect, disconnect, reconnect attempt
- Verify: Unit test with two processes communicating over localhost

#### Phase 7.2: Dedicated Server Executable `[ ]`

**Goal**: `bulwark-dedicated` runs as a headless server accepting network connections.

- Accept incoming QUIC connections
- Route `CreateRoom`/`JoinRoom` requests to Server room management
- Log all activity to stdout/file with `tracing`
- Configurable via `dedicated.toml` (port, max rooms, timeouts)
- Verify: Start Dedicated, connect with a test client, create and join a Room

#### Phase 7.3: App Network Client `[ ]`

**Goal**: App can connect to a remote Dedicated for multiplayer.

- "New Game (Multiplayer)": prompt for Dedicated server address (or use config default), connect via NetworkTransport, create Room, show Room code
- "Join Game": prompt for server address + Room code, connect and join
- All existing lobby and gameplay flows work over network (no code changes to game logic needed, just transport swap)
- Verify: Two App instances on the same machine connect to a Dedicated, play a game together

#### Phase 7.4: Reconnection `[ ]`

**Goal**: Clients can reconnect to a Room after disconnection.

- Server keeps disconnected Player in Room for configurable timeout
- On reconnect (same Player ID), Server sends full `RoomState` to resync
- Client handles reconnection gracefully (brief "Reconnecting..." overlay)
- If timeout expires, Player is removed; if Owner times out, ownership transfers
- Verify: Disconnect a client (kill process), restart, reconnect to same Room

### Milestone 8: Polish & Feel

#### Phase 8.1: Opponent Cursor Visibility `[ ]`

**Goal**: See what other players are doing in real-time.

- Cursor positions sent as part of regular Client->Server updates, Server broadcasts to all Clients
- Show other Players' cursors and ghost pieces (received from Server) as smaller, colored indicators
- During build phase: see opponents' ghost pieces hovering over their regions
- During battle phase: see opponents' crosshairs and cannon fire direction

#### Phase 8.2: Sound & Music `[ ]`

**Goal**: Full audio experience.

- Placeholder music tracks for: menu, build phase, battle phase, victory, defeat
- Sound effects for: piece place, piece rotate, invalid placement, cannon fire, cannonball flight, explosion, phase transition fanfare, menu navigation
- Volume respects settings
- Music crossfades between phases

#### Phase 8.3: Visual Polish `[ ]`

**Goal**: The game looks good.

- Terrain tiles with varied ground textures (not just flat colors)
- Water animation (simple tile animation or shader)
- Wall segments that connect visually (auto-tiling)
- Castle sprites
- Cannon sprites with directional facing
- Explosion particle effects
- Territory claim animation (color sweep flood fill)
- Phase announcement text (large centered text that fades)


### Milestone 9: Packaging & Distribution

#### Phase 9.1: Build Scripts & Packaging `[ ]`

**Goal**: One-command build and package for each OS.

- Cargo build script or Makefile/justfile for release builds
- Windows: directory with .exe files, .toml configs, data/ folder
- macOS: .app bundle with embedded binary, configs, data
- Linux: directory with binary, configs, data
- Strip debug symbols for release, optimize binary size

#### Phase 9.2: Default Configs & First-Run Experience `[ ]`

**Goal**: Game works out of the box with zero configuration.

- Ship sensible default .toml configs
- Auto-detect screen resolution for default window size
- Auto-generate Player ID on first launch
- Prompt for player name on first launch (or use OS username)

---

## Departures from Rampart

These are intentional differences from the original arcade game:

1. **Player count**: Up to 6 players (original supports 2-3). Grid scales larger for more players.
2. **Maps**: Hand-crafted maps (2 per player count, 10 total), stored as data files. Larger grids for higher player counts, camera zooms out to accommodate. Matches the original's approach of curated layouts.
3. **Networking**: Real-time multiplayer over the internet (original is local arcade only, or LAN on some ports).
4. **Input**: First-class keyboard, mouse, and gamepad support with configurable bindings (original uses trackball).
5. **Bots**: Computer-controlled players with difficulty levels (original single-player has enemy ships that attack, not competing builders).
6. **No enemy ships / grunts**: The original single-player has NPC ships that fire at your walls. Bulwark is purely player-vs-player (including Bots). This may be revisited if single-player feels too empty.

