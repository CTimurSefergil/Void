# Codename: Hiçlik (Void)
Hunt intelligent monsters in an ever-changing, infinite worlds.

# Core Features
## Özdevinimli Yaratım Algoritması (Ever-Changing World Creation Algorithm)
### Key Data Structures
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
### Main Algorithm Flow
  #### 1. World Manager
     Every nms:
      - Calculate player's grid position
      - Create cells in nxn grid around player
      - Remove cells too far from player (memory cleanup)
      - Update spatial index for fast lookups

  #### 2. Wave Function Collapse Process
    For each game frame:
    1. Initialize new cells with ALL tile possibilities
    2. Propagate constraints from collapsed cells to neighbors
    3. Find cell with lowest entropy (fewest possibilities)
    4. Randomly select tile based on weights
    5. Mark cell as collapsed, trigger more constraint propagation

  #### 3. Constraint Propagation Logic
    When a cell gets a tile type:
    For each neighbor direction:
      - Check what tiles are allowed next to this tile type
      - Remove invalid possibilities from neighbor's valid_tiles list
      - Update neighbor's entropy count
      - If neighbor has no valid tiles left: default to Ground

  #### 4. Tile Selection Algorithm
    Find all uncollapsed cells with minimum entropy
    Randomly pick one cell from this group
    Use weighted random selection from its valid_tiles
      - Ground: 30% weight (common)
      - Trees: 20% weight (decoration) 
      - Fountains: 50% weight (special structures)
      - Chests: 10% weight (rare)

      
  #### 5. Visual Rendering
    When cell.tile_type changes:
      - Load appropriate 3D model (ground.glb, tree.glb, etc.)
      - Apply correct rotation for fountain pieces
      - Scale model to cell size
      - Attach to game entity

### Performance Features
  * Spatial Indexing: O(1) cell lookups by position
  * Change Detection: Only update visuals when cells actually change
  * Throttled Updates: Expensive operations run every nms, not every frame
  * Memory Management: Auto-cleanup of distant cells
  * Batched Processing: Handle multiple constraint propagations per frame



## Advanced AI
### Key Data Structures
```rust
TheHumanAI {
    health: float                       // Current health points
    max_health: float                   // Maximum possible health
    emotional_state: EmotionalState     // Current emotion (Angry/Neutral/Depressed)
    current_behavior: AIBehavior        // Current action (Chasing/Escaping/Wandering/Begging)
    detection_range: float              // How far AI can see player
    movement_speed: float               // Base movement speed
    last_player_position: Vec3          // Memory of where player was last seen
    time_since_seen_player: float       // Seconds since last player sighting
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
### System Architecture (5 Independent Systems)
  #### 1. Health System
     Every frame:
      - Clamp health between 0 and max_health
      - Detect death/revival transitions
      - Log critical health events
      - Prevent invalid health states

  #### 2. Emotion System
    Every frame:
      - Calculate emotion from health percentage:
        * 0-30% health → Depressed (survival mode)
        * 70-100% health → Angry (aggressive mode)  
        * 30-70% health → Neutral (peaceful mode)
      - Update emotional_state if changed
      - Log emotion transitions

  #### 3. Behavior Decision System
    Every 200ms (performance optimized):
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

  #### 4. Movement Execution System
    Every frame:
      - Execute physical movement based on current behavior:
        * Wandering: Random direction, slow speed (0.5x)
        * Chasing: Move toward player, normal speed (1.0x)
        * Escaping: Move away from player/last known position, fast speed (1.5x)
        * Begging: No movement, face player
      - Handle rotation and orientation
      - Apply frame-rate independent movement

      
  #### 5. Speech System
    Every few seconds (timer-based):
      - 80% chance to speak when timer finishes
      - Select contextual dialogue based on emotion + behavior:
        * Depressed + Escaping: "Leave me alone...", "I can't take this anymore..."
        * Angry + Chasing: "Come here!", "You can't escape me!"
        * Angry + Begging: "I... I love you...", "That weapon scares me..."
        * Neutral + Begging: "Want to be friends?", "I like your company..."
      - Print selected dialogue to console

### Emergent Behavior Examples
#### Scenario: Player approaches healthy AI
    1. Health = 100% → Emotion = Angry
    2. Player detected → Behavior = Chasing
    3. AI chases aggressively, says "I'm going to get you!"
    
#### Scenario: Player pulls out weapon during chase
    1. Emotion = Angry, Behavior = Chasing
    2. Weapon detected → Behavior = Begging (fear override)
    3. AI stops chasing, faces player, says "That weapon scares me..."

#### Scenario: AI takes damage during fight
    1. Health drops to 20% → Emotion = Depressed
    2. Still can see player → Behavior = Escaping
    3. AI runs away at 1.5x speed, says "I just want to be left in peace..."

#### So on...

### Performance Features
  * Timer-based Updates: Expensive behavior calculations run every 200ms, not every frame
  * Modular Systems: Each system handles one responsibility independently
  * Memory Management: AI remembers last player position for realistic search behavior
  * State Caching: Only update when states actually change to avoid redundant processing

  
