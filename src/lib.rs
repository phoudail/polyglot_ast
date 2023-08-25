pub mod building;
pub mod context;
pub mod polyglot_tree;
/// Set of utilities and helpers to manipulate polyglot AST objects.
///
/// This module contains errors types, the Language enum as well as a few conversions functions.
pub mod util;
pub mod graal_utils;
pub mod tree_sitter_utils;

pub use polyglot_tree::polyglot_processor::{PolygotProcessor, TreePrinter};
pub use polyglot_tree::polyglot_zipper::PolyglotZipper;
pub use polyglot_tree::PolyglotTree;

/// An enumeration that represents all languages supported by this crate. Current options are Python, JavaScript and Java.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Python,
    JavaScript,
    /// Warning: Java language support is very partial and limited to string literal usage. Keep this in mind when writing your programs
    Java,
}

type SourceFilePath = String;

type SourceMap = std::collections::HashMap<std::path::PathBuf, (Language, String)>;
type FileMap = std::collections::HashMap<std::path::PathBuf, String>;

#[cfg(test)]
mod tests;
