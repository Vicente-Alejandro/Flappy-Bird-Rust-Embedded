<div align="center">

# 🐦 Flappy Bird — Bevy & Rust

**A modern Flappy Bird clone built with Rust and the Bevy game engine.**
Smooth physics, pixel-perfect collisions, and a growing feature set.

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/Bevy-0.15-blue?style=for-the-badge&logo=bevy)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Fork of](https://img.shields.io/badge/Fork%20of-Biped--Potato%2Fflappy__bird-lightgrey?style=for-the-badge&logo=github)](https://github.com/Biped-Potato/flappy_bird)

</div>

---

## 📖 About

This project is a fork of [Biped-Potato/flappy_bird](https://github.com/Biped-Potato/flappy_bird), a Flappy Bird clone written entirely in **Rust** using the **[Bevy](https://bevyengine.org/)** game engine.

The goal of this fork is to take the base implementation and push it further — improving gameplay feel, adding missing features (scoring, audio, menus), and using it as a learning playground for Bevy's ECS architecture.

---

## ✨ Features

### ✅ Implemented
- 🐦 Bird with gravity and flap physics
- 🟩 Procedurally spawned pipes
- 💥 Collision detection
- 🔄 Game reset on death

### 🚧 In Progress / Planned

| Feature | Status | Description |
|---|---|---|
| 🏆 Score system | `implemented` | Real-time score counter + high score persistence in `save.ron` |
| 🎬 Game states & Menus | `implemented` | Start screen, playing, and game over states |
| 🎵 Sound effects | `planned` | Flap, score, death and background music via `bevy_audio` |
| 📈 Difficulty scaling | `planned` | Increasing pipe speed and gap reduction over time |
| ✨ Visual polish | `planned` | Sprite animations, parallax background, particle effects |
| 🌐 WASM build | `planned` | Playable in the browser via WebAssembly |

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable — Bevy tracks close to the latest MSRV)
- `cargo` (bundled with Rust)

### Run locally

```bash
# Clone the repo
git clone https://github.com/<your-username>/flappy_bird.git
cd flappy_bird

# Run in development mode
cargo run

# Run with optimizations (recommended for smooth gameplay)
cargo run --release
```

> **Tip:** Add the following to `.cargo/config.toml` for faster compile times in development:
> ```toml
> [profile.dev]
> opt-level = 1
>
> [profile.dev.package."*"]
> opt-level = 3
> ```

---

## 🎮 Controls

| Input | Action |
|---|---|
| `Space` / `Left Click` | Flap |
| `R` | Restart (after death) |
| `Esc` | Quit |

---

## 🏗️ Architecture

The project follows Bevy's **Entity Component System (ECS)** pattern:

```
src/
├── main.rs          # App entry point, plugin registration
├── core/
│   ├── mod.rs       # CorePlugin, state machine initialization
│   ├── config.rs    # GameConfig parsing from .ron files
│   ├── state.rs     # GameState enum (MainMenu, Playing, GameOver)
│   └── save.rs      # Data persistence (High Score saving via Serde)
├── game/
│   ├── mod.rs       # GamePlugin, setup and cleanup logic
│   ├── player.rs    # Bird logic, physics, jumping
│   ├── environment.rs # Pipe spawning, scrolling, scoring
│   └── collision.rs # Decoupled collision detection
├── ui/
│   └── mod.rs       # UiPlugin, Main Menu, Game Over screen, HUD
└── hardware/        # (planned) Embedded systems integration
```

> Each game system is a **Bevy plugin**, keeping responsibilities isolated and the codebase modular.

---

## 🧰 Tech Stack

| | |
|---|---|
| **Language** | [Rust](https://www.rust-lang.org/) |
| **Game Engine** | [Bevy](https://bevyengine.org/) |
| **Architecture** | ECS (Entity Component System) |
| **Rendering** | `wgpu` (Vulkan / Metal / DX12 / WebGPU) |
| **Audio** *(planned)* | `bevy_audio` |

---

## 🤝 Credits

- Original implementation by [Biped-Potato](https://github.com/Biped-Potato/flappy_bird)
- Built on top of the [Bevy game engine](https://github.com/bevyengine/bevy) — free and open source forever

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.