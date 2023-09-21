//! # Polyglot AST
//!
//! Principle / objectives
//! - [ ] provide a simple API to parse a polyglot file and get his AST
//! - [ ] provide a simple API to query the AST
//! - [ ] provide a simple API to manipulate the AST
//!   
//! # Getting started
//! ```rust
//! use polyglot::prelude::*;
//! ```

pub mod building;
pub mod context;
pub mod graal_utils;
pub mod polyglot_tree;
pub mod tree_sitter_utils;
/// Set of utilities and helpers to manipulate polyglot AST objects.
///
/// This module contains errors types, the Language enum as well as a few conversions functions.
pub mod util;

pub mod builder;

pub use polyglot_tree::polyglot_processor::{PolygotProcessor, TreePrinter};
pub use polyglot_tree::polyglot_zipper::PolyglotZipper;
pub use polyglot_tree::PolyglotTree;

/// Default imports to facilitate access to novice users.
///
/// ```rust
/// GlobalContext;
///
/// // for each file
/// //     parse
///
/// // query stuff
/// ```
/// see also: examples/simple.rs
///
/// https://doc.rust-lang.org/reference/names/preludes.html
pub mod prelude {
    pub use super::Language;
    pub use super::PolyglotAstBuilder;
    pub use super::PolyglotTree;
    pub use super::PolyglotZipper;
    pub use super::TreePrinter;
    use super::*;
    pub use context::GlobalContext;
    pub use polyglot_tree::polyglot_processor::PolyglotTreeSerializer;
}
pub use builder::PolyglotAstBuilder;

/// An enumeration that represents all languages supported by this crate. Current options are Java and partial support for Python and JavaScript.
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

#[derive(Debug, Clone)]
pub struct RawParseResult {
    pub(crate) language: Language,
    errors: Vec<String>,
    pub cst: Option<tree_sitter::Tree>,
    source: std::sync::Arc<str>,
}
impl PartialEq for RawParseResult {
    fn eq(&self, other: &Self) -> bool {
        self.language == other.language && self.source == other.source
    }
}
impl Eq for RawParseResult {}
pub fn parse(code: std::sync::Arc<str>, language: Language) -> RawParseResult {
    let mut parser = tree_sitter::Parser::new();
    let ts_lang = util::language_enum_to_treesitter(&language);

    parser
            .set_language(ts_lang)
            .expect("Error loading the language grammar into the parser; if this error persists, consider reporting it to the library maintainers.");

    let tree = parser.parse(code.as_bytes(), None);
    RawParseResult {
        language,
        errors: vec![], // TODO
        cst: tree,
        source: code,
    }
    .into()
}
pub trait PolyStuff: std::fmt::Debug {
    fn kind(&self) -> building::PolyglotKind;
    fn lang(&self) -> Language;
    fn path(&self) -> Option<&std::path::Path>;
    fn source(&self) -> Option<&std::sync::Arc<str>>;
    fn position(&self) -> context::TopoOrder;
}

// trait not yet used in the code
pub trait PolyBuilder {
    fn polyglot_stuff(&self) -> Vec<Box<dyn PolyStuff>>;
}
impl<T> PolyBuilder for T
where
    T: building::StuffPerLanguage,
    T::UnsolvedUse: PolyStuff + 'static,
{
    fn polyglot_stuff(&self) -> Vec<Box<dyn PolyStuff>> {
        self.find_polyglot_uses()
            .into_iter()
            .map(|u| Box::new(u) as Box<dyn PolyStuff>)
            .collect()
        // TODO do the exports
    }
}

impl RawParseResult {
    pub fn compute_polyglot_stuff(&self) -> Option<Vec<Box<dyn PolyStuff + '_>>> {
        let cst = self.cst.as_ref()?;
        let cst: tree_sitter_utils::TreeSitterCST =
            tree_sitter_utils::into(Some(cst), &self.source);

        use crate::building::StuffPerLanguage;
        use building::PolyglotBuilding;
        match self.language {
            Language::Python => Some(vec![]),
            Language::JavaScript => todo!(),
            Language::Java => {
                let init = building::java::JavaBuilder::init(cst);
                let ana = building::java::DefaultRefAna::default();
                let v = init
                    .find_polyglot_uses()
                    .into_iter()
                    .map(|u| u.solve(&init, &ana))
                    .map(|u| Box::new(u) as Box<dyn PolyStuff>)
                    .collect();
                Some(v)
            }
        }
    }
}
