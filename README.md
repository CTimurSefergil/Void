# Void (codename: Hiçlik)

Hunt intelligent monsters in an ever-changing, infinite world. This is a 3D first-person game prototype in Rust with [Bevy](https://bevyengine.org) 0.16.

<!-- TODO: add a screenshot or GIF -->

## Highlights

- **Infinite world built by Wave Function Collapse in real time.** The world around the player is a 17 × 17 grid of cells. Each cell collapses into a tile (ground, tree, fountain pieces, chest…) according to adjacency rules and weights. New cells are created as the player moves and distant ones are freed.
- **Emotion-driven enemy AI.** An enemy's health sets its emotion (angry, neutral or depressed). Its emotion, whether the player is in range and whether the player has a weapon decide its behaviour: chase, escape, wander or beg, each with its own console dialogue. This is split into five single-purpose ECS systems.
- **Built with performance in mind.** A spatial index gives O(1) cell lookups, visuals only update when a cell actually changes, and cell spawning (every 200 ms) and AI decisions (every 0.5 s) run on timers instead of every frame.

## Running

You need Rust 1.85+ (edition 2024). On Linux, also install [Bevy's system dependencies](https://bevyengine.org/learn/quick-start/getting-started/setup/).

```sh
cargo run
```

The first build takes a while because Bevy is large. Dynamic linking is enabled to make later builds faster.

> **Note:** the enemy AI is currently turned off. To turn it on, uncomment `app.add_plugins(enemy_ai::plugin);` in `src/game/core_mechanics/mod.rs`. Keys 1–4 only work then.

| Key | Action |
|---|---|
| WASD / arrow keys | Move |
| Mouse | Look around |
| Esc | Release or re-grab the cursor (it's grabbed automatically when the window is focused) |
| 1 / 2 | Damage / heal the enemy (debug) |
| 3 | Toggle the player's weapon (debug) |
| 4 | Print the enemy's state (debug) |

Module names are Turkish: `oz_devinimli_yaratim` ("self-moving creation") is the world generator and `o_insan` ("that human") is the enemy AI.

## How it works

### Özdevinimli Yaratım Algoritması (Ever-Changing World Creation Algorithm)
#### Key Data Structures
```rust
Cell {
    position: (x, z)                    // Grid coordinates
    is_collapsed: boolean               // Has tile been chosen?
    tile_type: Optional<TileType>       // Final tile (ground, tree, fountain, etc.)
    valid_tiles: List<TileType>         // Still-possible tiles
    entropy: integer                    // Number of possibilities remaining
}

Rules {
    allowed_neighbors: Map              // Which tiles can be adjacent
    weights: Map                        // Spawn probability for each tile
}
```
#### Main Algorithm Flow
  ##### 1. World Manager
     Every 200 ms:
      - Calculate player's grid position
      - Create cells in a 17×17 grid around the player
      - Remove cells too far from player (memory cleanup)
      - Update spatial index for fast lookups

  ##### 2. Wave Function Collapse Process
    For each game frame:
    1. Initialize new cells with ALL tile possibilities
    2. Propagate constraints from collapsed cells to neighbors
    3. Find cell with lowest entropy (fewest possibilities)
    4. Randomly select tile based on weights
    5. Mark cell as collapsed, trigger more constraint propagation

  ##### 3. Constraint Propagation Logic
    When a cell gets a tile type:
    For each neighbor direction:
      - Check what tiles are allowed next to this tile type
      - Remove invalid possibilities from neighbor's valid_tiles list
      - Update neighbor's entropy count
      - If neighbor has no valid tiles left: default to Ground (for now)

  ##### 4. Tile Selection Algorithm
    Find all uncollapsed cells with minimum entropy
    Randomly pick one cell from this group
    Use weighted random selection from its valid_tiles (relative weights):
      - Ground: 0.3 (common)
      - Tree: 0.2 (decoration)
      - Chest: 0.1 (rare)
      - FountainCenter: 0.5, each fountain edge/corner piece ≈ 0.34

      
  ##### 5. Visual Rendering
    When cell.tile_type changes:
      - Load appropriate 3D model
      - Apply correct rotations
      - Scale model to cell size
      - Attach to game entity

#### World Performance Features
  * Spatial Indexing: O(1) cell lookups by position
  * Change Detection: Only update visuals when cells actually change
  * Throttled Updates: Cell spawning and despawning run every 200 ms, not every frame
  * Memory Management: Auto-cleanup of distant cells
  * Batched Processing: Handle multiple constraint propagations per frame



### Advanced AI
#### Key Data Structures
```rust
OInsanAI {
    health: float                       // Current health points
    max_health: float                   // Maximum possible health
    emotional_state: EmotionalState     // Current emotion (Angry/Neutral/Depressed)
    current_behavior: AIBehavior        // Current action (Chasing/Escaping/Wandering/Begging)
    detection_range: float              // How far AI can see player
    movement_speed: float               // Base movement speed
    last_player_position: Option<Vec3>  // Memory of where player was last seen
    time_since_seen_player: float       // Seconds since last player sighting
    behavior_update_timer: Timer        // Paces behaviour decisions
    speech_timer: Timer                 // Paces dialogue
}

EmotionalState {
    Angry,      // High health (70%+) - confident and aggressive
    Neutral,    // Medium health (30-70%) - calm and peaceful  
    Depressed   // Low health (0-30%) - scared and defensive
}

AIBehavior {
    Chasing,    // Actively pursuing player
    Escaping,   // Running away from player
    Wandering,  // Random exploration movement
    Begging     // Standing still, facing player
}
```
#### System Architecture (5 Single-Purpose Systems)
  ##### 1. Health System
     Every frame:
      - Clamp health between 0 and max_health
      - Detect death/revival transitions
      - Log critical health events
      - Prevent invalid health states

  ##### 2. Emotion System
    Every frame:
      - Calculate emotion from health percentage:
        * 0-30% health → Depressed (survival mode)
        * 70-100% health → Angry (aggressive mode)  
        * 30-70% health → Neutral (peaceful mode)
      - Update emotional_state if changed
      - Log emotion transitions

  ##### 3. Behavior Decision System
    Every 500 ms (performance optimized):
      - Gather situational context (player position, distance, weapon status)
      - Update AI memory if player is visible
      - Make behavior decision based on emotion + context:
        
        Depressed AI:
          - If can see player → Escape
          - If alone → Wander
            
        Angry AI:  
          - If can see player without weapon → Chase
          - If can see player with weapon → Beg (fear override)
          - If alone → Wander (search)
            
        Neutral AI:
          - Always → Beg (friendly)

  ##### 4. Movement Execution System
    Every frame:
      - Execute physical movement based on current behavior:
        * Wandering: Random direction, slow speed (0.5x)
        * Chasing: Move toward player, normal speed (1.0x)
        * Escaping: Move away from player/last known position, fast speed (1.5x)
        * Begging: No movement, face player
      - Handle rotation and orientation
      - Apply frame-rate independent movement

      
  ##### 5. Speech System
    Every 2 seconds (timer-based):
      - 80% chance to speak when timer finishes
      - Select contextual dialogue based on emotion + behavior:
        * Depressed + Escaping: "Leave me alone...", "I can't take this anymore..."
        * Angry + Chasing: "Come here!", "You can't escape me!"
        * Angry + Begging: "I... I love you...", "That weapon scares me..."
        * Neutral + Begging: "Want to be friends?", "I like your company..."
      - Print selected dialogue to console

#### Emergent Behavior Examples
##### Scenario: Player approaches healthy AI
    1. Health = 100% → Emotion = Angry
    2. Player detected → Behavior = Chasing
    3. AI chases aggressively, says "I'm going to get you!"
    
##### Scenario: Player pulls out weapon during chase
    1. Emotion = Angry, Behavior = Chasing
    2. Weapon detected → Behavior = Begging (fear override)
    3. AI stops chasing, faces player, says "That weapon scares me..."

##### Scenario: AI takes damage during fight
    1. Health drops to 20% → Emotion = Depressed
    2. Still can see player → Behavior = Escaping
    3. AI runs away at 1.5x speed, says "Please, I just want to be left in peace..."

And so on...

#### AI Performance Features
  * Timer-based Updates: Behavior decisions run every 500 ms, not every frame
  * Modular Systems: Each system handles one responsibility
  * Memory: AI remembers the player's last seen position (used as a fallback escape direction)
  * State Caching: Only update when states actually change to avoid redundant processing

## Known issues

- Fountain corners don't always form correctly.
- Areas you come back to are generated again, differently.
- Wandering picks a new direction every frame, so the enemy jitters in place.
- The player has no model yet.
