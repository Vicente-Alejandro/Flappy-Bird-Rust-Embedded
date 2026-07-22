# 2. Domain-Driven ECS Structure

Date: 2026-07-22

## Status

Accepted

## Context

Bevy is an ECS (Entity Component System) framework. While ECS enforces separating data (Components) from logic (Systems), it does not enforce a directory structure. 
Currently, all logic (`setup_level`, `update_bird`, `update_obstacles`, `collision_checks`) is tangled within a 200-line `src/main.rs`.
As we scale to include hardware integration (v0.5.0), UI, and Juice (v0.4.0), a single file will become a massive bottleneck and create unmaintainable spaghetti code.

## Decision

We will structure the project using Domain-Driven boundaries wrapped as Bevy Plugins.
The source directory (`src/`) will be strictly partitioned into the following domains:

- `core/`: Global configurations, telemetry, game state definitions, and generic utilities.
- `game/`: All game mechanics (the bird, the obstacles, collision math).
- `ui/`: All user interface elements (menus, score HUD).
- `hardware/`: Code responsible for the schematics rendering toggle, serial port parsing, and telemetry output.

Each domain will expose a top-level `Plugin` (e.g., `GamePlugin`) that `main.rs` registers.

## Consequences

**Positive:**
- Extremely modular codebase. We can disable the `UiPlugin` entirely and the game will still run.
- Easier to test specific systems in isolation.
- Prevents circular dependencies as long as `core` acts as the dependency sink.

**Negative:**
- Minor boilerplate overhead (each module needs a `pub struct XPlugin;` and `impl Plugin for XPlugin`).
- Requires strict discipline to not cross domain boundaries directly (e.g., `game` should not directly modify `ui` elements; they should communicate via Bevy Events or Resources).
