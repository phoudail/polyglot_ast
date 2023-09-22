//!
//!
//!
//!
//!

use crate::Language;

use super::util;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashMap, fs::File};
use tree_sitter::{Node, Parser, Tree};

pub mod polyglot_processor;
pub mod polyglot_zipper;

/// An Abstract Syntax Tree (AST) spanning across multiple languages.
///
///
///
#[derive(Debug, Clone)]
pub struct PolyglotTree {
    tree: Tree,
    code: std::sync::Arc<str>,
    // working_dir: PathBuf,
    language: Language,
    // node_to_subtrees_map: HashMap<usize, PolyglotTree>,
}

impl<'text> crate::tree_sitter_utils::TextProvider<'text> for PolyglotTree {
    type I = &'text str;
    type II = str;
    type N<'t> = tree_sitter::Node<'t>;
    fn text(&'text self, node: &Self::N<'_>) -> Self::I {
        &self.code[node.start_byte()..node.end_byte()]
    }
    fn t(&self, node: &Self::N<'_>) -> &Self::II {
        &self.code[node.byte_range()]
    }
}

impl<'text> crate::tree_sitter_utils::TextProvider<'text> for &'text PolyglotTree {
    type I = &'text str;
    type II = str;
    type N<'t> = tree_sitter::Node<'t>;
    fn text(&self, node: &Self::N<'_>) -> Self::I {
        &self.code[node.start_byte()..node.end_byte()]
    }
    fn t(&self, node: &Self::N<'_>) -> &Self::II {
        &self.code[node.byte_range()]
    }
}

impl TryFrom<&crate::RawParseResult> for PolyglotTree {
    type Error = ();
    fn try_from(value: &crate::RawParseResult) -> Result<Self, Self::Error> {
        value
            .cst
            .as_ref()
            .map(|tree| Self {
                tree: tree.clone(),
                code: value.source.clone(),
                language: value.language,
            })
            .ok_or(())
    }
}

struct PolyglotTreeBuilder<'a, 'b, 'ts> {
    tree: &'a PolyglotTree,
    node_tree_map: &'a mut HashMap<usize, PolyglotTree>,
    map_source: &'b mut SourceMap,
    map_file: &'b mut FileMap,
    node_stack: Vec<Node<'ts>>,
}

impl<'a: 'ts, 'b, 'ts> PolyglotTreeBuilder<'a, 'b, 'ts> {
    fn new(
        tree: &'a PolyglotTree,
        node_tree_map: &'a mut HashMap<usize, PolyglotTree>,
        map_source: &'b mut SourceMap,
        map_file: &'b mut FileMap,
    ) -> Self {
        Self {
            tree,
            node_tree_map,
            map_source,
            map_file,
            node_stack: vec![tree.tree.root_node()],
        }
    }
    fn get_language(&self) -> Language {
        self.tree.language
    }
    fn get_code(&self) -> &str {
        &self.tree.code
    }
    // fn get_working_dir(&self) -> &PathBuf {
    //     &self.tree.working_dir
    // }
}

impl PartialEq for PolyglotTree {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.language == other.language
        // && self.node_to_subtrees_map == other.node_to_subtrees_map
    }
}

impl Eq for PolyglotTree {}

type SourceMap = HashMap<String, (Language, String)>;
type FileMap = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ParsingResult {
    tree: Option<PolyglotTree>,
    errors: Vec<String>,
}

impl ParsingResult {
    pub fn tree(&self) -> &Option<PolyglotTree> {
        &self.tree
    }
}

impl From<crate::RawParseResult> for ParsingResult {
    fn from(value: crate::RawParseResult) -> Self {
        let tree = value.cst;
        let Some(tree) = tree else {
            return ParsingResult {
                tree: None,
                errors: vec![],
            };
        };

        let result = PolyglotTree {
            tree,
            code: value.source,
            // working_dir: PathBuf::new(),
            language: value.language,
            // node_to_subtrees_map: HashMap::new(),
        };
        Self {
            tree: Some(result),
            errors: vec![],
        }
    }
}

impl PolyglotTree {
    pub fn parse(code: std::sync::Arc<str>, language: Language) -> ParsingResult {
        let mut parser = Parser::new();
        let ts_lang = util::language_enum_to_treesitter(&language);

        parser
                .set_language(ts_lang)
                .expect("Error loading the language grammar into the parser; if this error persists, consider reporting it to the library maintainers.");

        let tree = parser.parse(code.as_bytes(), None);

        let Some(tree) = tree else {
                return ParsingResult {
                    tree: None,
                    errors: vec![],
                };
            };

        let mut result = PolyglotTree {
            tree,
            code,
            // working_dir: PathBuf::new(),
            language,
            // node_to_subtrees_map: HashMap::new(),
        };
        ParsingResult {
            tree: Some(result),
            errors: vec![],
        }
    }

    /// Given a path to a file and a Language, returns a PolyglotTree instance that represents the program written in the file.
    ///
    /// The provided AST is built recursively from variations of the `polyglot.eval` function call in different languages,
    /// and can be traversed across language boundaries.
    ///
    /// Returns None if there was a problem while reading the file or during the parsing phase,
    /// which can happen either due to timeout or messing with the parser's cancellation flags;
    /// refer to the `tree_sitter::Parser::parse()` documentation for more information.
    ///
    /// If there is an error while reading the file, this method will "soft fail" and return None while printing a message to io::stderr.
    ///
    /// # Arguments
    ///
    /// - `path` A PathBuf to the file containing the code.
    /// - `language` The Language variant that the file at `path` is written in.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use polyglot_ast::PolyglotTree;
    /// use polyglot_ast::util::Language;
    ///
    /// let file = PathBuf::from("TestSamples/export_x.py");
    /// let tree: PolyglotTree = PolyglotTree::from_path(file, Language::Python).expect("This test file exists");
    ///
    /// let file = PathBuf::from("this_file_does_not_exist.py");
    /// assert!(PolyglotTree::from_path(file, Language::Python).is_none());
    /// ```
    ///
    /// # Panics
    ///
    /// This method can only panic if there is a problem while loading the language grammar into the parser,
    /// either in this call or subsequent recursive calls to build subtrees.
    /// This can only happen if tree_sitter and the grammars are of incompatible versions;
    /// either refer to the `tree_sitter::Parser::set_language()` documentation or directly contact polyglot_ast maintainers if this method keeps panicking.
    pub fn from_path(path: PathBuf, language: Language) -> Option<PolyglotTree> {
        todo!()
    }

    /// Applies the given processor to the tree, starting from the root of the tree.
    /// For more information, refer to the PolyglotProcessor trait documentation.
    pub fn apply(&self, processor: &mut impl polyglot_processor::PolygotProcessor) {
        processor.process(polyglot_zipper::PolyglotZipper::<PolyglotTree>::new(self));
    }

    /// Internal function to get a node's source code.
    fn node_to_code(&self, node: Node) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }

    /// Internal function to get the root node of the tree.
    fn root_node(&self) -> Node {
        self.tree.root_node()
    }
}

enum LinkError {
    LanguageNotHandled(String),
    LanguageMissing,
    CodeMissing,
    CodeFileMissing,
    FileMissing,
    PathMissing,
    ParametersMissing,
}
