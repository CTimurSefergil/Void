// ============================================================================
// 📜 ODYRULES - Rule System for Procedural Generation
// ============================================================================
//
// This module contains all the rules that govern how tiles can be placed
// next to each other in the procedural world generation system. Different
// rule sets create different types of environments.
//
// 📋 BEST PRACTICE: Rule-based generation
// - Separate rule definitions from generation algorithms
// - Modular rule system allows different world types
// - Commons module provides shared tile definitions
// - Each rule set creates coherent, themed environments

//pub mod building_rules;   // Rules for architectural structures (future)
pub mod commons;            // Shared tile definitions and utilities
//pub mod dungeon_rules;    // Rules for underground/enclosed spaces (future)
pub mod open_space_rules;   // Rules for outdoor, natural environments
