// ============================================================================
// 🧩 COMMONS - Shared Tile Definitions and Rules Framework
// ============================================================================
//
// This module defines the fundamental building blocks for the procedural
// generation system: tile types, directions, and rule interfaces that
// all generation systems use.
//
// 📋 BEST PRACTICE: Shared foundation for all rule systems
// - Common tile definitions prevent inconsistencies
// - Direction system enables spatial relationship rules
// - Trait-based rules allow different generation strategies
// - EnumIter enables automatic iteration over all tile types

use bevy::platform::collections::HashMap;
use strum_macros::EnumIter;

// ============================================================================
// 🎯 SECTION 1: TILE TYPE DEFINITIONS (World Building Blocks)
// ============================================================================

/// All possible tile types that can be placed in the world
/// 
/// 📋 BEST PRACTICE: Comprehensive tile system
/// - EnumIter allows automatic iteration over all tiles
/// - Copy + Clone for efficient passing around
/// - Hash + Eq for use in collections and comparisons
/// - Debug for development and troubleshooting
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
pub enum TileType {
    // Basic Environment Tiles
    Ground,              // Basic walkable ground surface
    Tree,                // Natural obstacle/decoration
    Chest,               // Interactive object (treasure, rocks, etc.)
    
    // Fountain Structure System (Multi-part connected structure)
    FountainCenter,      // Central fountain piece
    FountainCorner1,     // Corner piece - rotation variant 1
    FountainCorner2,     // Corner piece - rotation variant 2  
    FountainCorner3,     // Corner piece - rotation variant 3
    FountainCorner4,     // Corner piece - rotation variant 4
    FountainEdge1,       // Edge piece - rotation variant 1
    FountainEdge2,       // Edge piece - rotation variant 2
    FountainEdge3,       // Edge piece - rotation variant 3
    FountainEdge4,       // Edge piece - rotation variant 4
}

// ============================================================================
// 🧭 SECTION 2: DIRECTION SYSTEM (Spatial Relationships)
// ============================================================================

/// Cardinal directions for tile adjacency rules
/// 
/// 📋 BEST PRACTICE: Clear directional relationships
/// - Front/Back and Right/Left provide intuitive spatial reasoning
/// - Hash + Eq enable use in collections for rule lookup
/// - Copy for efficient parameter passing
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    Front,  // Positive Z direction
    Back,   // Negative Z direction
    Right,  // Positive X direction
    Left,   // Negative X direction
}

/// Array of all directions for iteration
/// 
/// 📋 BEST PRACTICE: Constant arrays for iteration
/// - Enables checking rules in all directions
/// - Consistent ordering for predictable behavior
pub const DIRECTIONS: [Direction; 4] = [
    Direction::Front,
    Direction::Back,
    Direction::Right,
    Direction::Left,
];

/// Direction to coordinate vector mapping
/// 
/// 📋 BEST PRACTICE: Coordinate system integration  
/// - Maps abstract directions to concrete grid coordinates
/// - Enables spatial math for neighbor finding
/// - (x, z) coordinates match 3D world space
pub const DIRECTION_VECTORS: [(Direction, (i32, i32)); 4] = [
    (Direction::Front, (0, 1)),   // +Z
    (Direction::Back, (0, -1)),   // -Z
    (Direction::Right, (1, 0)),   // +X
    (Direction::Left, (-1, 0)),   // -X
];

// ============================================================================
// 🎨 SECTION 3: RULE SYSTEM INTERFACE (Generation Strategy)
// ============================================================================

/// Trait defining the interface for all rule systems
/// 
/// 📋 BEST PRACTICE: Trait-based rule system
/// - Different rule sets can implement different generation strategies
/// - allowed_neighbors: Defines which tiles can be adjacent
/// - weights: Controls probability of each tile type appearing
/// - Lifetime parameters allow efficient reference borrowing
pub trait Rules {
    /// Returns the adjacency rules for all tile types
    /// 
    /// 📋 DESIGN NOTE: Nested HashMap structure
    /// - Outer key: The tile type being placed
    /// - Inner key: Direction to check for neighbors
    /// - Value: List of tile types allowed in that direction
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>>;
    
    /// Returns the spawn weights for all tile types
    /// 
    /// 📋 DESIGN NOTE: Weight-based probability
    /// - Higher weight = more likely to appear
    /// - Enables creating rare vs common tiles
    /// - Allows fine-tuning of world generation feel
    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32>;
}
