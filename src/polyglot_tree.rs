use super::util;
use super::util::Language;
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
    code: String,
    working_dir: PathBuf,
    language: Language,
    node_to_subtrees_map: HashMap<usize, PolyglotTree>,
}

struct PolyglotTreeBuilder<'a, 'b, 'ts> {
    tree: &'a PolyglotTree,
    node_tree_map: &'a mut HashMap<usize, PolyglotTree>,
    map_source: &'b mut SourceMap,
    map_file: &'b mut FileMap,
    node_stack: Vec<Node<'ts>>,
}

impl<'a, 'b, 'ts> PolyglotTreeBuilder<'a, 'b, 'ts> {
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
    fn get_working_dir(&self) -> &PathBuf {
        &self.tree.working_dir
    }
}


/////////////// NEW STRUCTURE ////////////////////// (also in antoher class -> choose the best place)
pub trait StructLanguage {
    fn get_polyglot_call(&self) -> String;
    fn make_subtree(&mut self, node_data: String);
}

impl StructLanguage for PolyglotTree {
    fn get_polyglot_call(&self) -> String {
        format!("Polyglot.call({}, {})", self.code, self.language)
    }

    fn make_subtree(&mut self, node_data: String) {
        let node = Node::new(node_data);
        self.node_to_subtrees_map.insert(node.id, node);
    }
}




impl PartialEq for PolyglotTree {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.language == other.language
            && self.node_to_subtrees_map == other.node_to_subtrees_map
    }
}

impl Eq for PolyglotTree {}

type SourceMap = HashMap<String, (Language, String)>;
type FileMap = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParsingResult {
    tree: Option<PolyglotTree>,
    errors: std::sync::Arc<Vec<String>>,
}

impl ParsingResult {
    pub fn tree(&self) -> &Option<PolyglotTree> {
        &self.tree
    }
}

impl PolyglotTree {
    /// Given a program's code and a Language, returns a PolyglotTree instance that represents the program.
    ///
    /// The provided AST is built recursively from variations of the `polyglot.eval` function call in different languages, and can be traversed across language boundaries.
    ///
    /// Returns None if there was a problem during the parsing phase, which can happen either due to timeout or messing with the parser's cancellation flags.
    /// If you are not using tree-sitter in your program, you can safely assume this method will never return None;
    /// refer to the `tree_sitter::Parser::parse()` documentation for more information.
    ///
    /// # Arguments
    ///
    /// - `code` The code snippet to build the AST from, provided as any object that can be converted to a string.
    /// For proper use, ensure that `code.to_string()` would provide a syntactically correct code snippet.
    /// - `language` The Language variant that `code` is written in.
    ///
    /// # Examples
    ///
    /// ```
    /// use polyglot_ast::PolyglotTree;
    /// use polyglot_ast::util::Language;
    ///
    /// let empty_tree: PolyglotTree = PolyglotTree::from("", Language::Python).expect("Python is a supported language");
    /// let tree: PolyglotTree = PolyglotTree::from("print(x*42)", Language::Python).expect("Python is a supported language");
    /// let py_js_tree: PolyglotTree = PolyglotTree::from("import polyglot\nprint(x*42)\npolyglot.eval(language=\"js\", string=\"console.log(42)\"", Language::Python).expect("Python is a supported language");
    /// ```
    ///
    /// # Panics
    ///
    /// This method can only panic if there is a problem while loading the language grammar into the parser, either in this call or subsequent recursive calls to build subtrees.
    /// This can only happen if tree_sitter and the grammars are of incompatible versions;
    /// either refer to the `tree_sitter::Parser::set_language()` documentation or directly contact polyglot_ast maintainers if this method keeps panicking.
    pub fn from(code: impl ToString, language: Language) -> ParsingResult {
        let code = code.to_string();

        let mut parser = Parser::new();
        let ts_lang = util::language_enum_to_treesitter(&language);

        parser
            .set_language(ts_lang)
            .expect("Error loading the language grammar into the parser; if this error persists, consider reporting it to the library maintainers.");

        let tree = parser.parse(code.as_str(), None);

        let Some(tree) = tree else {
            return ParsingResult {
                tree: None,
                errors: std::sync::Arc::new(vec![]),
            };
        };

        let mut result = PolyglotTree {
            tree,
            code,
            working_dir: PathBuf::new(),
            language,
            node_to_subtrees_map: HashMap::new(),
        };

        let tree = Tree { /* initialisation de Tree */ };
        let code = "some_code".to_string();
        let working_dir = PathBuf::from("/path/to/working/directory");
        let language = Language::Rust; // Remplacez par le langage approprié
        let node_to_subtrees_map = HashMap::new();

        let polyglot_tree = PolyglotTree::new();
        let mut node_tree_map = HashMap::new();
        let mut map_source = SourceMap::new();
        let mut map_file = FileMap::new();
        let mut params = PolyglotTreeBuilder::new(
            &polyglot_tree,
            &mut node_tree_map,
            &mut map_source,
            &mut map_file,
        );

        result.build_polyglot_tree(params);
        result.node_to_subtrees_map = node_tree_map; // set the map after its built
        ParsingResult {
            tree: Some(result),
            errors: std::sync::Arc::new(vec![]),
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
        let file = path.clone();
        let code = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "Warning: unable to create tree for file {} due to the following error: {e}",
                    file.to_str()?
                );
                return None;
            }
        };

        let mut parser = Parser::new();
        let ts_lang = util::language_enum_to_treesitter(&language);

        parser
            .set_language(ts_lang)
            .expect("Error loading the language grammar into the parser; consider verifying your versions of the grammar and tree-sitter are compatible.");

        let tree = parser.parse(code.as_str(), None)?;

        let mut result = PolyglotTree {
            tree,
            code,
            working_dir: file.parent()?.to_path_buf(),
            language,
            node_to_subtrees_map: HashMap::new(),
        };

        let mut map = HashMap::new();
        let mut map_source: SourceMap = HashMap::new();
        let mut map_file = HashMap::new();
        result.build_polyglot_tree(&mut map, &mut map_source, &mut map_file);
        result.node_to_subtrees_map = map;

        Some(result)
    }

    /// Internal function to build a polyglot tree, which sets a specific working directory for the built subtree.
    /// This is used when a polyglot file has a polyglot call to raw code, to ensure any subsequent calls would properly locate files.
    ///
    /// # Arguments
    ///
    /// - `code` The code snippet to build the AST from, provided as any object that can be converted to a string.
    /// For proper use, ensure that `code.to_string()` would provide a syntactically correct code snippet.
    /// - `language` The Language variant that the file at `path` is written in.
    /// - `working_dir` a PathBuf of the parent directory of the file currently being processed.
    ///
    /// # Panics
    ///
    /// This method can only panic if there is a problem while loading the language grammar into the parser,
    /// either in this call or subsequent recursive calls to build subtrees.
    /// This can only happen if tree_sitter and the grammars are of incompatible versions;
    /// either refer to the `tree_sitter::Parser::set_language()` documentation or directly contact polyglot_ast maintainers if this method keeps panicking.
    fn from_directory(
        code: impl ToString,
        language: Language,
        working_dir: PathBuf,
    ) -> Option<PolyglotTree> {
        let code = code.to_string();

        let mut parser = Parser::new();
        let ts_lang = util::language_enum_to_treesitter(&language);

        parser
            .set_language(ts_lang)
            .expect("Error loading the language grammar into the parser; consider verifying your versions of the grammar and tree-sitter are compatible.");

        let tree = parser.parse(code.as_str(), None)?;

        let mut result = PolyglotTree {
            tree,
            code,
            working_dir,
            language,
            node_to_subtrees_map: HashMap::new(),
        };

        //let mut params
        result.build_polyglot_tree(params);
        result.node_to_subtrees_map = params.map;
        Some(result)
    }

    /// Applies the given processor to the tree, starting from the root of the tree.
    /// For more information, refer to the PolyglotProcessor trait documentation.
    pub fn apply(&self, processor: &mut impl polyglot_processor::PolygotProcessor) {
        processor.process(polyglot_zipper::PolyglotZipper::from(self))
    }

    /// Internal function to get a node's source code.
    fn node_to_code(&self, node: Node) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }

    /// Internal function to get the root node of the tree.
    fn root_node(&self) -> Node {
        self.tree.root_node()
    }

    /// Internal function to start building the polyglot mappings and subtrees.
    fn build_polyglot_tree(&self, params: PolyglotTreeBuilder) -> Result<(), LinkError> {
        let root = self.tree.root_node();
        //self.build_polyglot_links(node_tree_map, root, map_source, map_file)?; // we get the root, and then call the recursive function
        self.build_polyglot_links(params);
        Ok(())
    }

    /// Internal recursive function that iterates over the nodes in the tree, and builds all subtrees as well as the polyglot link map.
    fn build_polyglot_links(&self, params: PolyglotTreeBuilder) -> Result<(), LinkError> {
        // TODO first

        let mut stack = vec![self.root_node()];

        while let Some(node) = stack.pop() {
            if node.kind() == "local_variable_declaration" {
                // TODO extract function
                for i in 0..node.named_child_count() {
                    if self.node_to_code(node.child(i).unwrap()) == "Source" {
                        //pair contains (language, file)
                        let pair = node
                            .child(1)
                            .and_then(|n| n.child(2))
                            .and_then(|n| n.child(0))
                            .and_then(|n| n.child(3))
                            .ok_or(LinkError::ParametersMissing)?;
                        let source = self
                            .node_to_code(node.child(1).unwrap().child(0).unwrap())
                            .to_string();
                        let lang =
                            self.node_to_code(pair.child(1).ok_or(LinkError::LanguageMissing)?);
                        let lang = util::strip_quotes(lang);
                        let language = util::language_string_to_enum(&lang)
                            .map_err(|_| LinkError::LanguageNotHandled(lang))?;
                        let file = self
                            .node_to_code(pair.child(3).ok_or(LinkError::CodeMissing)?)
                            .to_string();
                        //insert (source_variable, (language, file))
                        params.map_source.insert(source, (language, file));
                    }
                    if self.node_to_code(node.child(i).unwrap()) == "File" {
                        let file = self
                            .node_to_code(
                                node.child(1)
                                    .and_then(|n| n.child(0))
                                    .ok_or(LinkError::FileMissing)?,
                            )
                            .to_string();

                        let path: &str = self.node_to_code(
                            node.child(1)
                                .and_then(|n| n.child(2))
                                .and_then(|n| n.child(2))
                                .and_then(|n| n.child(1))
                                .ok_or(LinkError::PathMissing)?,
                        );

                        let path = util::strip_quotes(path);
                        //insert (file_variable, path of the file)
                        params.map_file.insert(file, path);
                    }
                }
            }
            if self.is_polyglot_eval_call(node) {
                if !self.make_subtree(
                    params.node_tree_map,
                    params.map_source,
                    params.map_file,
                    self.root_node(),
                ) {
                    // If building the subtree failed,
                    // we want to soft fail (eg. not panic) to avoid interrupting the tree building.
                    // Eventually, this should be made into a proper Error,
                    // but for now for debugging purposes it just prints a warning.
                    eprintln!(
                        "Warning: unable to make subtree for polyglot call at position {}",
                        node.start_position()
                    )
                }
            } else {
                if let Some(child) = node.child(0) {
                    stack.push(child);
                }
                if let Some(sibling) = node.next_sibling() {
                    stack.push(sibling);
                }
            }
        }
        Ok(())
    }
    fn make_subtree(
        &self,
        node_tree_map: &mut HashMap<usize, PolyglotTree>,
        map_source: &mut SourceMap,
        map_file: &mut FileMap,
        node: Node,
    ) -> bool {
        let subtree: PolyglotTree;
        let result: Option<PolyglotTree> = match self.language {
            // delegate to language specific subfunction
            Language::Python => self.make_subtree_python(&node),
            Language::JavaScript => self.make_subtree_js(&node),
            Language::Java => self.make_subtree_java(&node, map_source, map_file),
        };

        subtree = match result {
            Some(t) => t,
            None => return false,
        };

        node_tree_map.insert(node.id(), subtree);

        true // signal everything went right
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
