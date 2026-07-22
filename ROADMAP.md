# 🚀 Flappy Bird Rust Embedded - Development Roadmap

This document defines the architectural vision and the iteration plan to evolve this project from a simple "single-file clone" into a **professional-grade software engineering piece**.

The goal of this roadmap is not only to create a fun game but to build a **high-value addition for a professional portfolio**, demonstrating advanced mastery of Rust, Data-Oriented Architecture (ECS with Bevy), solid engineering practices, and hardware/embedded systems integration.

---

## 🏗️ v0.2.0 & v0.2.1 - Architecture & Engineering Foundations (Design Phase) - COMPLETED
**Goal:** Establish a scalable, predictable, and professional foundation. Before writing mechanics code, we prepare the architectural groundwork.

*   ✅ **Architecture Decision Records (ADRs):** Implement ADRs to formally document why and how we choose certain patterns (e.g., how we will handle persistence or network/serial communication).
*   ✅ **Domain-Driven Directory Structure:** Define a strict folder hierarchy that separates responsibilities (Core, Rendering, UI, Physics, Hardware).
*   ✅ **Continuous Integration (CI/CD):** 
    *   Configured GitHub Actions for manual triggers.
    *   **(v0.2.1)** Built a custom Rust microservice (`cargo-qc`) with traceability logs for completely local, cost-free, automated quality control.
*   ✅ **Deterministic Physics:** Implementation of a *Fixed Timestep* to guarantee that the game's physics behave identically regardless of the machine's framerate.
*   ✅ **Data-Driven Configuration:** Designed the system so that variables like Gravity, Flap Force, and Speed are loaded from external files (`.ron`), allowing game tuning without recompiling.
*   ✅ **Telemetry and Logging:** Integrated the `tracing` crate to emit structured logs and profile system performance.

---

## 🧩 v0.3.0 - ECS Refactor & Core Logic (Systems Phase)
**Goal:** Translate the v0.2.0 design into clean code using the ECS (Entity Component System) paradigm to its fullest potential.

*   **Plugin Modularization:**
    *   `PlayerPlugin`: Logic exclusive to the bird.
    *   `EnvironmentPlugin`: Procedural generation and object pooling for pipes.
    *   `CollisionPlugin`: Collision detection fully decoupled from movement logic.
    *   `UiPlugin`: Pure user interface logic.
*   **Game State Machine:** Implement clean state transitions: `MainMenu` ➔ `Playing` ➔ `Paused` ➔ `GameOver`.
*   **Data Persistence:** Local high score system using `serde` for disk serialization.
*   **Quality of Life (QoL) - Base Gameplay:**
    *   **Pause:** Ability to pause the game with `ESC` or `P`.
    *   **Gamepad Support:** Integrate controller input via `gilrs` (already in dependencies) to play with Xbox/PlayStation controllers.

---

## 🧃 v0.4.0 - Juice & Polish (Game Feel Phase)
**Goal:** Turn the game into a premium, tactile experience. Demonstrate knowledge of shaders, animation math, and visual/audio feedback.

*   **Procedural Animation:** 
    *   Mathematical *Squash and Stretch* when jumping and falling, making it feel organic.
    *   Dynamic rotation based on velocity derivatives (improving the basic rotation with smooth interpolation).
*   **Particle Systems:** 
    *   Dust/wind trails when flapping.
    *   Pixel/feather explosions upon collision (using a particle engine like `bevy_hanabi`).
*   **Camera Feedback:**
    *   *Screen Shake* with variable intensity based on collision type.
*   **Spatial & Interactive Audio:**
    *   Stereo panning when pipes pass from right to left.
    *   Dynamic music where tempo or intensity increases as the score goes up.
*   **Post-Processing (Visuals):**
    *   Options to enable *Bloom* or CRT/Retro Shaders to give it a unique aesthetic identity.
*   **Quality of Life (QoL) - Advanced Gameplay:**
    *   **Coyote Time:** A split-second grace period allowing a jump just before colliding, reducing player frustration.

---

## 🔌 v0.5.0 - "Schematics" & Embedded Systems (The Unique Selling Proposition)
**Goal:** Integrate the game with hardware and technical representations, fulfilling the repository's name and giving it a unique engineering value.

*   **"Schematic" Visual Mode:**
    *   A toggle that switches all rendering in real-time to an "Engineering Blueprint / CAD" view. Textures are replaced by vector lines, measurements, angles, and force vectors drawn over the bird.
*   **Hardware Integration (Input):**
    *   Support for Serial communication (RS-232/USB). Allow the bird to jump upon receiving a signal from a microcontroller (e.g., a physical button connected to an Arduino/ESP32).
*   **External Telemetry (Output):**
    *   Export real-time game data (altitude, velocity, collisions) via UDP or Serial, so it can be visualized on an external OLED display, an oscilloscope, or a Grafana dashboard.
*   **"Simulator" Mode:**
    *   Implementation of an autopilot bot (rudimentary AI or PID controller) that plays automatically by calculating trajectories, enabling the game to run autonomously in technical exhibitions or fairs.

---

## 🔮 Future (v?.?.0+)
*   Asynchronous Multiplayer (Ghosts of other players).
*   WebAssembly (Wasm) port to play directly from an online portfolio.
