// ============================================================================
// 🌳 OPEN SPACE RULES - Natural Environment Generation Rules
// ============================================================================
//
// This module defines the specific rules for generating open, natural
// environments with ground, trees, decorative objects, and multi-part
// fountain structures. It implements the Rules trait to provide adjacency
// constraints and spawn weights for Wave Function Collapse generation.
//
// 📋 BEST PRACTICE: Detailed rule specification
// - Each tile type has specific neighbor constraints
// - Complex structures (fountains) have precise connection rules
// - Weight system controls rarity and distribution
// - Resource pattern makes rules globally available

use bevy::{ecs::resource::Resource, platform::collections::HashMap};
use strum::IntoEnumIterator;

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::{
    DIRECTIONS, Direction, Rules, TileType,
};

// ============================================================================
// 📊 SECTION 1: RESOURCE DEFINITION (Rule Data Structure)
// ============================================================================

/// Resource containing all rules for open space generation
/// 
/// 📋 BEST PRACTICE: Comprehensive rule storage
/// - allowed_neighbors: Defines which tiles can be adjacent in each direction
/// - all_tiles: Complete list for initialization and iteration
/// - weights: Probability weights for random tile selection
/// - Resource pattern makes rules available to all systems
#[derive(Resource, Debug)]
pub struct OpenSpaceRules {
    /// Adjacency rules: TileType -> Direction -> Allowed neighbors
    /// 
    /// 📋 DESIGN NOTE: Nested HashMap structure
    /// - First level: Which tile we're placing
    /// - Second level: Which direction we're checking
    /// - Value: List of tile types allowed in that direction
    pub allowed_neighbors: HashMap<TileType, HashMap<Direction, Vec<TileType>>>,
    
    /// Complete list of all available tile types
    /// 
    /// 📋 DESIGN NOTE: Used for cell initialization
    /// - New cells start with all tiles as possibilities
    /// - Rules progressively eliminate invalid options
    pub all_tiles: Vec<TileType>,
    
    /// Spawn probability weights for each tile type
    /// 
    /// 📋 DESIGN NOTE: Controls world generation feel
    /// - Higher weights = more common tiles
    /// - Lower weights = rare, special tiles
    /// - Fine-tuning creates desired world aesthetics
    pub weights: HashMap<TileType, f32>,
}

// ============================================================================
// 🎯 SECTION 2: TRAIT IMPLEMENTATION (Rules Interface)
// ============================================================================

impl Rules for OpenSpaceRules {
    /// 🎯 INTERFACE: Neighbor Rules Access
    /// Returns reference to adjacency rules for efficient querying
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>> {
        &self.allowed_neighbors
    }

    /// 🎯 INTERFACE: Weight Rules Access  
    /// Returns reference to spawn weights for probability calculations
    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32> {
        &self.weights
    }
}

// ============================================================================
// 🛠️ SECTION 3: UTILITY METHODS (Rule Setup Helpers)
// ============================================================================

impl OpenSpaceRules {
    /// 🎯 UTILITY: Set Same Rules for All Directions
    /// Helper function to assign the same neighbor list to all four directions
    /// 
    /// 📋 BEST PRACTICE: DRY principle for symmetric tiles
    /// - Some tiles (like trees) can be placed next to the same things in all directions
    /// - Reduces code duplication and potential for errors
    /// - Makes rule maintenance easier
    fn set_all_directions(rules_map: &mut HashMap<Direction, Vec<TileType>>, tiles: Vec<TileType>) {
        for dir in DIRECTIONS.iter() {
            rules_map.insert(*dir, tiles.clone());
        }
    }
}

// ============================================================================
// 🏗️ SECTION 4: DEFAULT IMPLEMENTATION (Rule Definition)
// ============================================================================

impl Default for OpenSpaceRules {
    /// 🎯 RULE DEFINITION: Complete Open Space Rule Set
    /// Defines all adjacency rules and weights for natural environment generation
    /// 
    /// 📋 BEST PRACTICE: Comprehensive rule specification
    /// - Each tile type has carefully considered neighbor constraints
    /// - Fountain system uses precise connection rules for proper assembly
    /// - Weight system balances common vs rare elements
    /// - Comments explain the spatial logic for each rule set
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        let mut rules_map = HashMap::new();

        // ====================================================================
        // 🌱 GROUND TILE RULES: Basic Walkable Surface
        // ====================================================================
        
        // 📋 DESIGN NOTE: Ground tile adjacency rules
        // - Ground can connect to other ground (continuous surfaces)
        // - Trees and chests can be placed on ground (natural placement)
        // - Specific fountain pieces can connect to ground (structure integration)
        // - Different fountain pieces allowed in different directions for proper assembly
        
        rules_map.insert(
            Direction::Front,
            vec![
                TileType::Ground,           // More ground in front
                TileType::Tree,             // Tree in front
                TileType::Chest,            // Chest in front
                TileType::FountainCorner3,  // Bottom-left fountain corner
                TileType::FountainCorner4,  // Bottom-right fountain corner
                TileType::FountainEdge4,    // Bottom fountain edge
            ],
        );
        rules_map.insert(
            Direction::Back,
            vec![
                TileType::Ground,
                TileType::Tree,
                TileType::Chest,
                TileType::FountainCorner1,  // Top-left fountain corner
                TileType::FountainCorner2,  // Top-right fountain corner
                TileType::FountainEdge1,    // Top fountain edge
            ],
        );
        rules_map.insert(
            Direction::Right,
            vec![
                TileType::Ground,
                TileType::Tree,
                TileType::Chest,
                TileType::FountainCorner1,  // Top-left fountain corner
                TileType::FountainCorner3,  // Bottom-left fountain corner
                TileType::FountainEdge3,    // Left fountain edge
            ],
        );
        rules_map.insert(
            Direction::Left,
            vec![
                TileType::Ground,
                TileType::Tree,
                TileType::Chest,
                TileType::FountainCorner2,  // Top-right fountain corner
                TileType::FountainCorner4,  // Bottom-right fountain corner
                TileType::FountainEdge3,    // Left fountain edge
            ],
        );
        allowed_neighbors.insert(TileType::Ground, rules_map.clone());
        rules_map.clear();

        // ====================================================================
        // 🌲 TREE TILE RULES: Natural Decorative Elements
        // ====================================================================
        
        // 📋 DESIGN NOTE: Tree placement rules
        // - Trees are decorative and don't connect to fountain structures
        // - Can be placed next to ground, other trees, and chests
        // - Same rules in all directions (symmetric placement)
        
        OpenSpaceRules::set_all_directions(
            &mut rules_map,
            vec![TileType::Ground, TileType::Tree, TileType::Chest],
        );
        allowed_neighbors.insert(TileType::Tree, rules_map.clone());
        rules_map.clear();

        // ====================================================================
        // 📦 CHEST TILE RULES: Interactive Objects
        // ====================================================================
        
        // 📋 DESIGN NOTE: Chest placement rules
        // - Chests are standalone interactive objects
        // - Same placement rules as trees (decorative elements)
        // - Don't integrate with fountain structures
        
        OpenSpaceRules::set_all_directions(
            &mut rules_map,
            vec![TileType::Ground, TileType::Tree, TileType::Chest],
        );
        allowed_neighbors.insert(TileType::Chest, rules_map.clone());
        rules_map.clear();

        // ====================================================================
        // ⛲ FOUNTAIN CENTER RULES: Multi-Part Structure Core
        // ====================================================================
        
        // 📋 DESIGN NOTE: Fountain center connection rules
        // - Center connects only to fountain edges and other centers
        // - Each direction connects to specific edge types for proper assembly
        // - No direct connection to ground (edges provide that interface)
        
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge1, TileType::FountainCenter], // Top edge or more center
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge4, TileType::FountainCenter], // Bottom edge or more center
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge2, TileType::FountainCenter], // Right edge or more center
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge3, TileType::FountainCenter], // Left edge or more center
        );
        allowed_neighbors.insert(TileType::FountainCenter, rules_map.clone());
        rules_map.clear();

        // ====================================================================
        // 🔲 FOUNTAIN CORNER RULES: Structure Corner Pieces
        // ====================================================================
        
        // 📋 DESIGN NOTE: Fountain corner connection logic
        // - Each corner has specific orientation and connection rules
        // - Corner1 = Top-left, Corner2 = Top-right, etc.
        // - Internal sides connect to edges/corners, external sides connect to ground
        // - Precise rules ensure proper fountain assembly
        
        // FOUNTAIN CORNER1 (Top-left corner)
        rules_map.insert(Direction::Front, vec![TileType::Ground]); // External side to ground
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge3, TileType::FountainCorner3], // Internal: connects to left edge or bottom-left corner
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge1, TileType::FountainCorner2], // Internal: connects to top edge or top-right corner
        );
        rules_map.insert(Direction::Left, vec![TileType::Ground]); // External side to ground
        allowed_neighbors.insert(TileType::FountainCorner1, rules_map.clone());
        rules_map.clear();

        // FOUNTAIN CORNER2 (Top-right corner)
        rules_map.insert(Direction::Front, vec![TileType::Ground]); // External side to ground
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge2, TileType::FountainCorner4], // Internal: connects to right edge or bottom-right corner
        );
        rules_map.insert(Direction::Right, vec![TileType::Ground]); // External side to ground
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge1, TileType::FountainCorner1], // Internal: connects to top edge or top-left corner
        );
        allowed_neighbors.insert(TileType::FountainCorner2, rules_map.clone());
        rules_map.clear();

        // FOUNTAIN CORNER3 (Bottom-left corner)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge3, TileType::FountainCorner1], // Internal: connects to left edge or top-left corner
        );
        rules_map.insert(Direction::Back, vec![TileType::Ground]); // External side to ground
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge4, TileType::FountainCorner4], // Internal: connects to bottom edge or bottom-right corner
        );
        rules_map.insert(Direction::Left, vec![TileType::Ground]); // External side to ground
        allowed_neighbors.insert(TileType::FountainCorner3, rules_map.clone());
        rules_map.clear();

        // FOUNTAIN CORNER4 (Bottom-right corner)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge2, TileType::FountainCorner2], // Internal: connects to right edge or top-right corner
        );
        rules_map.insert(Direction::Back, vec![TileType::Ground]); // External side to ground
        rules_map.insert(Direction::Right, vec![TileType::Ground]); // External side to ground
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge4, TileType::FountainCorner3], // Internal: connects to bottom edge or bottom-left corner
        );
        allowed_neighbors.insert(TileType::FountainCorner4, rules_map.clone());
        rules_map.clear();

        // ====================================================================
        // ━ FOUNTAIN EDGE RULES: Structure Edge Pieces
        // ====================================================================
        
        // 📋 DESIGN NOTE: Fountain edge connection logic
        // - Edge pieces form the perimeter of fountain structures
        // - Each edge type can expand in specific directions
        // - Internal side connects to center/other edges, external to ground
        // - Expandable edges allow fountains of different sizes
        
        // FOUNTAIN EDGE1 (Top edge - expandable horizontally)
        rules_map.insert(Direction::Front, vec![TileType::Ground]); // External: to ground
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainCenter, TileType::FountainEdge4], // Internal: to center or opposite edge
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge1, TileType::FountainCorner2], // Expandable: more edge or corner
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge1, TileType::FountainCorner1], // Expandable: more edge or corner
        );
        allowed_neighbors.insert(TileType::FountainEdge1, rules_map.clone());
        rules_map.clear();

        // FOUNTAIN EDGE2 (Right edge - expandable vertically)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge2, TileType::FountainCorner2], // Expandable: more edge or corner
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge2, TileType::FountainCorner4], // Expandable: more edge or corner
        );
        rules_map.insert(Direction::Right, vec![TileType::Ground]); // External: to ground
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainCenter, TileType::FountainEdge3], // Internal: to center or opposite edge
        );
        allowed_neighbors.insert(TileType::FountainEdge2, rules_map.clone());
        rules_map.clear();

        // FOUNTAIN EDGE3 (Left edge - expandable vertically)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge3, TileType::FountainCorner1], // Expandable: more edge or corner
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge3, TileType::FountainCorner3], // Expandable: more edge or corner
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainCenter, TileType::FountainEdge2], // Internal: to center or opposite edge
        );
        rules_map.insert(Direction::Left, vec![TileType::Ground]); // External: to ground
        allowed_neighbors.insert(TileType::FountainEdge3, rules_map.clone());
        rules_map.clear();

        // FOUNTAIN EDGE4 (Bottom edge - expandable horizontally)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainCenter, TileType::FountainEdge1], // Internal: to center or opposite edge
        );
        rules_map.insert(Direction::Back, vec![TileType::Ground]); // External: to ground
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge4, TileType::FountainCorner4], // Expandable: more edge or corner
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge4, TileType::FountainCorner3], // Expandable: more edge or corner
        );
        allowed_neighbors.insert(TileType::FountainEdge4, rules_map.clone());
        rules_map.clear();

        // ====================================================================
        // ⚖️ WEIGHT SYSTEM: Spawn Probability Configuration
        // ====================================================================
        
        // 📋 DESIGN NOTE: Weight-based spawn probability
        // - Higher weights = more common in generated worlds
        // - Ground: Common base surface (30%)
        // - Tree: Common decoration (20%)  
        // - Chest: Uncommon interactive (10%)
        // - FountainCenter: Special structure trigger (50% - high to start fountains)
        // - Fountain pieces: Various weights to balance structure assembly
        
        let mut weights = HashMap::new();
        for tile in TileType::iter() {
            let weight = match tile {
                TileType::Ground => 0.3,          // Common: basic surface
                TileType::Tree => 0.2,            // Common: natural decoration
                TileType::Chest => 0.1,           // Uncommon: special objects
                TileType::FountainCenter => 0.5,  // High: triggers fountain creation
                
                // Fountain piece weights - slightly different for variety
                TileType::FountainCorner1 => 0.34567,
                TileType::FountainCorner2 => 0.3456,
                TileType::FountainCorner3 => 0.345,
                TileType::FountainCorner4 => 0.34,
                TileType::FountainEdge1 => 0.339,
                TileType::FountainEdge2 => 0.338,
                TileType::FountainEdge3 => 0.337,
                TileType::FountainEdge4 => 0.336,
            };
            weights.insert(tile, weight);
        }

        // ====================================================================
        // 📋 FINAL ASSEMBLY: Complete Rule Set
        // ====================================================================
        
        OpenSpaceRules {
            allowed_neighbors,
            all_tiles: vec![
                TileType::Ground,
                TileType::Tree,
                TileType::Chest,
                TileType::FountainCenter,
                TileType::FountainCorner1,
                TileType::FountainCorner2,
                TileType::FountainCorner3,
                TileType::FountainCorner4,
                TileType::FountainEdge1,
                TileType::FountainEdge2,
                TileType::FountainEdge3,
                TileType::FountainEdge4,
            ],
            weights,
        }
    }
}
