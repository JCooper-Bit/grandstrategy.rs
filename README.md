# GS Engine Architecture (Grand Strategy Engine)

This document explains the internal architecture of GS Engine, a Rust + Bevy + Lua-based framework for building CK3-style grand strategy games.

The goal is to separate:

- Engine logic (Rust)
- Gameplay logic (Lua)
- Simulation rules (data-driven systems)

---

# Core Philosophy

GS Engine is built around one principle:

> “Rust simulates the world. Lua defines the world.”

Rust is responsible for:
- Performance
- Determinism
- ECS simulation
- Rendering

Lua is responsible for:
- Game rules
- Events
- AI behavior
- Modding

---

# High-Level Architecture

```
+----------------------+
|      Bevy Engine     |
|  (Rendering + ECS)   |
+----------+-----------+
           |
           v
+----------------------+
|   Simulation Layer   |
| (Time, World State)  |
+----------+-----------+
           |
           v
+----------------------+
|    Lua Runtime       |
| (Game Logic Layer)   |
+----------------------+
```

---

# Core Systems

## 1. Time System

The game runs on a tick-based simulation.

- 1 tick = smallest simulation unit
- ticks aggregate into days, months, years
- Lua can hook into time events

Example:

```lua
function on_year_tick()
    print("A new year begins")
end
```

---

## 2. World System

The world is represented using ECS entities:

- Provinces
- Characters
- Dynasties
- Armies

Each entity is stored in Bevy ECS and exposed to Lua via controlled APIs.

---

## 3. Event System (Core Gameplay Loop)

Events drive gameplay.

Structure:

- Trigger → Condition check
- Effect → State changes
- Scope → Target entity

Example Lua event:

```lua
Event = {
    id = "birth_event",

    trigger = function(character)
        return character.age == 0
    end,

    effect = function(character)
        print(character.name .. " was born")
    end
}
```

---

## 4. Character System

Characters are simulation units with:

- Traits
- Stats
- Relationships
- Titles

Rust stores data, Lua defines behavior.

---

## 5. Map System

The world map is built from:

- Heightmaps (terrain generation)
- Province graph (logical regions)
- Borders (dynamic ownership)

Rendering is handled by Bevy.

---

## 6. Lua Runtime Layer

Lua is embedded as a single-threaded scripting runtime.

Responsibilities:

- Game logic execution
- Event evaluation
- AI decisions
- Modding API

Rules:

- Lua must not directly mutate ECS
- All changes go through engine API

---

# Engine API (Lua Exposure)

Planned API surface:

```lua
Game.spawn_character()
Game.get_character(id)
Game.set_title_holder(title, character)
Game.trigger_event(event_id)
Game.get_province(id)
```

---

# Game Loop Flow

Each frame:

1. Bevy updates ECS
2. Time system advances tick
3. Events are evaluated
4. Lua update() is called
5. Engine processes queued commands

---

# Modding System

Mods are loaded from:

```
game/
```

Structure:

- main.lua
- systems/
- events/
- characters/

Each file can register gameplay logic into the engine.

---

# Performance Model

- Rust handles all heavy computation
- Lua handles logic only
- Communication is via controlled API calls

No direct shared mutable state.

---

# Future Extensions

- Multiplayer determinism layer
- Save/load serialization system
- AI macro-planner system
- UI scripting layer in Lua

---

# Vision

GS Engine aims to become a modular grand strategy simulation framework where:

> “The engine simulates reality. The mod defines hi