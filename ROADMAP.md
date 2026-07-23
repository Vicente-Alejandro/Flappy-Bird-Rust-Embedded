# 🚀 Flappy Bird Rust Embedded — Development Roadmap

This document defines the architectural vision and the full iteration plan to evolve this project from a solid ECS foundation into a **production-quality, portfolio-defining piece of software engineering**.

The goal is not simply to ship a Flappy Bird clone. The goal is to build something **technically ambitious, visually distinctive, and genuinely fun** — a game that demonstrates mastery of Rust, Bevy ECS, data-oriented design, hardware integration, and professional software engineering practices.

> **Current version**: v0.4.0 — Juice & Polish ⏳
> **Last updated**: 2026-07-22
> **Legend**: ✅ Complete · 🔄 In Progress · ⏳ Pending · 🚫 Deferred

---

## ✅ v0.2.0 & v0.2.1 — Architecture & Engineering Foundations (COMPLETE)

**Goal:** Establish a scalable, predictable, and professional foundation before any mechanics code.

- ✅ Architecture Decision Records (ADRs)
- ✅ Domain-Driven Directory Structure (Core / Rendering / UI / Physics / Hardware)
- ✅ CI/CD — GitHub Actions + custom `cargo-qc` microservice with traceability logs
- ✅ Deterministic Physics — Fixed Timestep implementation
- ✅ Data-Driven Configuration — game vars loaded from `.ron` files at runtime
- ✅ Telemetry and Logging — `tracing` crate with structured, profiled output

---

## ✅ v0.3.0 — ECS Refactor & Core Logic (COMPLETE)

**Goal:** Translate the v0.2.0 design into clean ECS code using the full power of Bevy's plugin system.

- ✅ Plugin Modularization
  - ✅ `PlayerPlugin` — bird logic
  - ✅ `EnvironmentPlugin` — procedural pipe generation and object pooling
  - ✅ `CollisionPlugin` — collision detection fully decoupled from movement
  - ✅ `UiPlugin` — pure UI logic
- ✅ Game State Machine — `MainMenu` → `Playing` → `Paused` → `GameOver`
- ✅ Data Persistence — high score via `serde` disk serialization
- ✅ Pause mechanic (`ESC` / `P`)
- ✅ Gamepad input architecture — structured and ready for hardware testing

---

## 🧃 v0.4.0 — Juice & Polish (Game Feel Phase) ⏳

**Goal:** Transform the functional game into a premium, tactile experience. This version is about making every single interaction feel *satisfying*. Polish is not cosmetic — it is the difference between a game people play once and a game people remember.

### Procedural Animation
- [x] **Squash & Stretch** — mathematical deformation on jump impulse and fall apex using `Transform` scale interpolation; makes the bird feel alive, not rigid
- [x] **Velocity-derivative rotation** — bird nose angle driven by `velocity.y` with `lerp` smoothing; replaces the current basic rotation with physically intuitive tilt
- [x] **Wing flap animation** — sprite sheet cycling tied to flap input, independent of physics update loop

### Particle Systems (`bevy_hanabi`)
- [ ] **Flap trail** — dust/air disturbance particles emitted on every jump input; opacity and spread driven by flap frequency
- [ ] **Collision burst** — feather/pixel explosion on death; particle count and velocity driven by impact speed at collision moment
- [ ] **Pipe pass sparkle** — subtle score-confirmation particle on successful gap navigation

### Camera & Screen Feedback
- [ ] **Screen shake** — trauma-based shake system (not random; deterministic decay curve); intensity parameterized by collision speed and configurable in `.ron`
- [ ] **Camera lead** — camera slightly anticipates bird movement direction, giving the player more reaction time without changing physics

### Audio System (`bevy_kira_audio`)
- [ ] **Spatial stereo panning** — pipes panned right-to-left as they cross the bird's x-position; adds depth without 3D
- [ ] **Layered dynamic music** — base loop always playing; percussion layer fades in after score 10; intensity layer (faster BPM or additional instruments) after score 25
- [ ] **Reactive SFX** — distinct sounds for: flap, score, near-miss (pipe within N pixels), collision, new high score
- [ ] **Audio accessibility toggle** — mute SFX / mute music independently, persisted to disk with `serde`

### Post-Processing & Shaders (Custom WGSL)
- [ ] **Bloom** — mild bloom on bright pixels (bird, score numbers); toggle in settings
- [ ] **CRT / Retro shader** — scanlines + slight barrel distortion + chromatic aberration; activated via settings toggle; demonstrates custom shader pipeline knowledge
- [ ] **Day/Night cycle background** — procedural sky gradient that shifts from dawn → noon → dusk → night as score increases; purely aesthetic, no gameplay impact

### Quality of Life — Advanced Gameplay
- [ ] **Coyote Time** — configurable grace window (default: 80ms) allowing a jump input just before ground/pipe collision; dramatically reduces cheap deaths without affecting skilled play; value in `.ron`
- [ ] **Input buffer** — flap input buffered for ~80ms so rapid taps during brief physics lag are never dropped
- [ ] **Score combo multiplier** — consecutive pipe passes without near-misses build a visual multiplier (×1 → ×2 → ×3); resets on near-miss; shown in HUD
- [ ] **Difficulty curve** — pipe speed and gap size adjust dynamically based on score thresholds (configurable in `.ron`); prevents the game from plateauing

### Settings & Accessibility Screen
- [ ] **In-game settings panel** — accessible from pause menu; controls: SFX volume, music volume, bloom toggle, CRT toggle, coyote time on/off
- [ ] **Resolution and fullscreen toggle**
- [ ] **All settings persisted to disk** via `serde` alongside the high score

---

## 🌍 v0.5.0 — World Variation & Progression (Depth Phase) ⏳

**Goal:** Give players a reason to keep playing beyond pure score-chasing. Introduce environmental variety and a lightweight progression system that makes each run feel different — without changing the core one-button mechanic that makes Flappy Bird work.

*This version is the one most people skip. It is also the version that separates a tech demo from an actual game.*

### Procedural Biome System (`BiomePlugin`)
- [ ] **Biome definitions in `.ron`** — each biome defines: background gradient colors, obstacle skin, ambient particle set, music layer, and transition score threshold
- [ ] **4 base biomes** with distinct visual and audio identity:
  - [ ] `Overworld` — classic sky, green pipes, chirpy audio (start biome)
  - [ ] `Underground` — cave walls, stone pillars, echo-filtered audio, torch particle ambient
  - [ ] `Underwater` — blue gradient, coral/kelp obstacles, reverb audio, bubble ambient particles
  - [ ] `Circuit` — dark background, neon wire obstacles, electronic music layer, grid-line shader overlay
- [ ] **Smooth biome transition** — crossfade shader blend over ~3 seconds between biomes; no hard cut
- [ ] **Biome unlock log** — `BiomeSeen` component persisted per-run; unlocked biomes shown in main menu

### Environmental Hazards (`HazardPlugin`)
- [ ] **Moving pipes** — pipe pairs that oscillate vertically at configurable amplitude and frequency (in `.ron`); introduced after score 15
- [ ] **Narrow gap events** — occasional pipe pair with a gap 30% smaller than normal; telegraphed by a color flash 1 second before it enters screen
- [ ] **Wind zones** — horizontal regions that apply a lateral force to the bird; visualized as directional particle streams; introduced in `Circuit` biome
- [ ] **All hazards configurable** — amplitude, frequency, trigger score, force strength in `.ron`

### Collectibles & Run Variety (`CollectiblePlugin`)
- [ ] **Coins** — spawned between pipes; collecting 3 in a run without dying extends a "shield" (one hit absorbed); purely optional
- [ ] **Speed boost token** — speeds up the bird temporarily; gap size compensates automatically; risky/rewarding
- [ ] **Shield token** — one-hit invincibility for 5 seconds; brief visual indicator (pulsing outline)
- [ ] **Collectible spawn rates** in `.ron`; can be set to 0 to disable all collectibles

### Progression & Meta Systems
- [ ] **Run statistics** — per-run tracking: pipes cleared, near-misses, collectibles, biomes visited, peak speed; shown on death screen
- [ ] **Lifetime statistics** — cumulative totals persisted to disk: total runs, total pipes cleared, furthest run, favorite biome (most time spent)
- [ ] **Achievement system** — 12 achievements defined in `.ron`; examples: "First 10 pipes", "Reach Underground", "Survive Wind Zone", "Score without collecting anything", "Beat your high score 5 times"
- [ ] **Achievement notification** — non-blocking toast overlay on unlock; does not interrupt gameplay

---

## 🔌 v0.6.0 — Schematics & Embedded Systems (The Unique Selling Proposition) ⏳

**Goal:** Integrate the game with hardware and engineering visualizations. This is what makes the repository worth studying — a game that bridges interactive software and physical hardware.

### Schematic Visual Mode (`SchematicPlugin`)
- [ ] **Real-time rendering toggle** — `F1` key or settings menu switches ALL rendering to "Engineering Blueprint" mode without pausing the game or reloading assets
- [ ] **Blueprint render pipeline** — custom WGSL shader replaces texture sampling with:
  - [ ] Solid fill replaced by wireframe + hatching
  - [ ] Force vectors drawn over the bird (gravity arrow down, flap impulse arrow up, velocity vector forward)
  - [ ] Collision hitbox outlines rendered as dashed lines
  - [ ] Pipe gap measurement annotation (renders the gap distance in pixels as a dimension line)
- [ ] **Physics overlay** — optional HUD layer showing live values: `velocity.y`, `gravity`, `flap_force`, `timestep_acc`; values from the ECS components directly
- [ ] **Schematic mode persisted** in settings

### Hardware Integration — Input (`SerialInputPlugin`)
- [ ] **Serial port abstraction** — `SerialInput` resource wrapping `serialport` crate; non-blocking reads on a dedicated thread; sends `FlapEvent` to Bevy on signal received
- [ ] **Arduino/ESP32 compatibility** — documented wiring: digital pin HIGH → flap; works with any RS-232/USB-Serial device
- [ ] **Graceful degradation** — if no serial device connected, game runs on keyboard/gamepad without error; `SerialInput` resource is optional
- [ ] **Serial device selector** — settings panel lists available COM ports; selected port persisted to disk

### Hardware Integration — Output (`TelemetryOutputPlugin`)
- [ ] **UDP telemetry stream** — every fixed timestep emits a `TelemetryFrame` as JSON over UDP: `{ ts, bird_y, velocity_y, alive, score, biome }`
- [ ] **Serial telemetry stream** — same `TelemetryFrame` over serial at configurable baud rate; formatted for OLED display consumption
- [ ] **Grafana dashboard template** — included in `docs/grafana/`; connects to a local UDP→InfluxDB bridge; documents how to visualize a live game session on a real Grafana instance
- [ ] **Telemetry enable/disable** in settings; output target (UDP/Serial/both) configurable in `.ron`

### Autopilot / Simulator Mode (`SimulatorPlugin`)
- [ ] **PID controller bot** — reads `bird_y`, `velocity_y`, and next pipe gap position from ECS; computes flap decision using a tuned PID loop; runs as a normal Bevy system replacing player input
- [ ] **Simulator UI** — shows PID parameters live (Kp, Ki, Kd); values adjustable via egui sliders without recompilation
- [ ] **Bot vs Player split-screen** (stretch goal) — two independent physics instances running side-by-side; bot on left, player on right
- [ ] **Activation** — `F2` key or settings; shows clear "AUTOPILOT ACTIVE" overlay so it cannot be mistaken for normal play

---

## 🌐 v0.7.0 — WebAssembly & Accessibility (Reach Phase) ⏳

**Goal:** Make the game playable in a browser from a portfolio URL, and ensure it is accessible to the widest possible audience. This is what converts a GitHub repo viewer into someone who actually plays the game.

### WebAssembly Port
- [ ] **`wasm32-unknown-unknown` build target** — `Cargo.toml` feature flags separating desktop-only (serial, UDP) from Wasm-compatible features
- [ ] **`wasm-pack` + GitHub Pages CI** — automated deploy to `gh-pages` branch on every push to `main`; URL in repo description
- [ ] **Wasm-compatible asset loading** — all `.ron` configs and audio assets loaded via async HTTP fetch in Wasm context
- [ ] **Touch input** — `TouchPlugin` mapping tap/touch to `FlapEvent`; makes the Wasm version playable on mobile browsers
- [ ] **Wasm performance** — target 60 FPS on a mid-range phone browser; profile and optimize draw calls and particle counts for Wasm constraints

### Localization (`LocalizationPlugin`)
- [ ] **`i18n` system** — all UI strings externalized to `assets/i18n/<lang>.ron`; language selected at runtime
- [ ] **2 base languages**: `en` (English) and `es` (Spanish)
- [ ] **Language selector** in settings; persisted to disk
- [ ] **Font support** — ensure the chosen font covers all glyphs for supported languages

### Accessibility
- [ ] **Colorblind modes** — 3 presets: Deuteranopia, Protanopia, Tritanopia; implemented as a post-process shader that remaps color space
- [ ] **Reduced motion mode** — disables screen shake, reduces particle density to 20%, disables camera lead; all in one toggle
- [ ] **High contrast mode** — increases bird/pipe outline weight and saturates colors for low-vision players
- [ ] **All accessibility options** in settings panel, persisted to disk

---

## 🏆 v1.0.0 — Stable Release (Production Quality) ⏳

**Goal:** A complete, polished, bug-free, well-documented `1.0.0` that can be shared publicly, played in a browser, and submitted as a portfolio piece with pride. This is not a feature release — it is a quality and completeness release.

### Final Polish Pass
- [ ] **Full gameplay balance review** — play-test every biome, every hazard, every difficulty threshold; tune `.ron` values based on data, not gut
- [ ] **Death replay** — on game over, a 2-second slow-motion replay of the final collision; uses recorded `Transform` history (ring buffer, last 120 frames); shows the player exactly what happened
- [ ] **New high score ceremony** — dedicated animation sequence (fanfare, score counter roll, particle burst) when a personal best is beaten; distinct from normal game over
- [ ] **Main menu polish** — animated background (the game running in autopilot bot mode behind the menu); title card with bloom; version number displayed

### Documentation
- [ ] **`README.md` complete rewrite** — installation, controls, feature showcase GIFs, hardware wiring diagram, Grafana setup, architecture overview
- [ ] **`docs/ARCHITECTURE.md`** — full ECS plugin map, data flow diagram, system dependency graph
- [ ] **`docs/HARDWARE.md`** — Arduino/ESP32 wiring guide with photos or diagrams, baud rate config, serial protocol spec
- [ ] **`docs/CONTRIBUTING.md`** — how to add a new biome (the canonical extension point), how to add an achievement, how to run tests
- [ ] **All ADRs finalized** — any decisions made during v0.4–v0.7 that were not documented, captured retroactively

### Testing & Quality Gate
- [ ] **Integration test suite** — automated tests for: state machine transitions, fixed timestep determinism (same seed → same outcome), collision detection correctness, serialization round-trip (save → load → verify)
- [ ] **Performance benchmark** — `criterion` bench for: physics update loop, collision system, particle system; baseline recorded and committed to `benches/`
- [ ] **Wasm smoke test** — automated headless browser test (via `playwright` or `wasm-bindgen-test`) that loads the game, plays 5 frames, and confirms no JS exceptions
- [ ] **Zero `clippy` warnings** — `cargo clippy --all-targets --all-features -- -D warnings` passes clean
- [ ] **Zero `rustfmt` diffs** — `cargo fmt --check` passes clean

### Release Artifacts
- [ ] **GitHub Release `v1.0.0`** with annotated tag following `semver-tagging` skill
- [ ] **Attached binaries** — pre-compiled releases for: `x86_64-pc-windows-gnu`, `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin` (via CI cross-compilation)
- [ ] **Live Wasm demo** — deployed to GitHub Pages; URL linked from `README.md` and GitHub repo description
- [ ] **`CHANGELOG.md`** complete from v0.1.0 through v1.0.0

---

## 🔮 Post-v1.0.0 (Unscheduled)

These items are intentionally deferred until after `1.0.0` is stable and complete.

- [ ] Asynchronous multiplayer — ghost runs of other players (replay-based, not real-time)
- [ ] Level editor — allow players to define custom pipe sequences in `.ron` and share them
- [ ] Mobile native build — `android` and `ios` targets via Bevy's mobile support
- [ ] Leaderboard — optional cloud high score via a lightweight REST API

---

## Dependency map (version gates)

```
v0.2.0 ──► v0.3.0 ──► v0.4.0 ──► v0.5.0 ──► v0.6.0 ──► v0.7.0 ──► v1.0.0
  │                       │           │
  │                       │           └── Biome system depends on
  │                       │               shader work from v0.4.0 (CRT/post-processing)
  │                       │
  │                       └── Audio system (`bevy_kira_audio`) in v0.4.0
  │                           is reused by spatial telemetry in v0.6.0
  │
  └── Fixed Timestep (v0.2.0) is the foundation for
      determinism testing in v1.0.0
```

**Hard gates:**
- v0.5.0 cannot begin until v0.4.0 shader work (CRT/post-process pipeline) is done — biome transitions use the same pipeline
- v0.6.0 Serial Input can be developed independently, but Schematic Mode requires the shader pipeline from v0.4.0
- v0.7.0 Wasm build requires feature-flagging the serial/UDP code added in v0.6.0
- v1.0.0 cannot begin until all prior versions are marked complete