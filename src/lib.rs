/// Set of utilities and helpers to manipulate polyglot AST objects.
///
/// This module contains errors types, the Language enum as well as a few conversions functions.
pub mod util;

/// The main module of the project.
/// 
/// This module contains the PolyglotTree struct, which is the main object used to build and interact with polyglot ASTs.
pub mod polyglot_tree;
pub use polyglot_tree::polyglot_processor::{PolygotProcessor, TreePrinter};
pub use polyglot_tree::polyglot_zipper::PolyglotZipper;
pub use polyglot_tree::PolyglotTree;

#[cfg(test)]
mod tests;
