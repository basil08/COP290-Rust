pub mod display;
pub mod function;
pub mod graph;
pub mod parser;
pub mod util_ext;
pub mod display_ext;
pub mod function_ext;
pub mod graph_ext;
pub mod parser_ext;
// # Spreadsheet System
//
// A modular and extensible spreadsheet engine implemented in Rust, designed for efficient formula evaluation,
// dependency tracking, and interactive features. This project supports both standard and extended spreadsheet modes.
//
// ## Features
// - Arithmetic expression parsing and evaluation
// - Cell dependency management using graph structures
// - Cycle detection and topological sorting for recalculation
// - Built-in functions: `SUM`, `AVG`, `MIN`, `MAX`, `STDEV`, etc.
// - Extended support for `String`, `Float`, and typed cell operations
// - Support for undo/redo functionality (in extended mode)
// - Autofill feature with pattern detection (AP, GP, etc.)
//
// ## Modules
//
// - [`display`](display): Handles rendering of spreadsheet UI (standard mode).
// - [`function`](function): Implements arithmetic and helper functions (standard mode).
// - [`graph`](graph): Tracks dependencies and handles formula recalculations (standard mode).
// - [`parser`](parser): Parses and evaluates expressions with support for integer cells (standard mode).
// - [`util_ext`](util_ext): Shared utilities used in extended evaluation logic.
// - [`display_ext`](display_ext): Enhanced rendering logic with extended features.
// - [`function_ext`](function_ext): Extended functions for float and string cell values.
// - [`graph_ext`](graph_ext): Extended dependency tracking with range support and advanced recalculation.
// - [`parser_ext`](parser_ext): Extended parser for multiple cell types and string operations.
//
// ## Getting Started
// To use the spreadsheet engine, select either the standard or extended mode through your binary's entry point.
// This selection allows dynamic inclusion of features depending on user needs.
//
// ## Example
// ```bash
// cargo run -- 10 10         # Run in standard mode
// cargo run -- 10 10 -extended   # Run in extended mode
// ```
//
// ## Author
// Built by Vani Gupta, 2025  
// Modular design encourages future contributions and easy feature expansion.
