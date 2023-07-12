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
#[derive(Debug)]
pub struct PolyglotTree {
    tree: Tree,
    code: String,
    working_dir: PathBuf,
    language: Language,
    node_to_subtrees_map: HashMap<usize, PolyglotTree>,
}

type SourceMap = HashMap<String, (Language, String)>;
type FileMap = HashMap<String, String>;

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
    pub fn from(code: impl ToString, language: Language) -> Option<PolyglotTree> {
        println!("tree - code pris en entrée {}", code.to_string());
        // le print du langage pris en paramètre en fonctionne pas
        // println!("tree - langage pris en entrée : {language}");

        let code = code.to_string();

        let mut parser = Parser::new();
        let ts_lang = util::language_enum_to_treesitter(&language);

        parser
            .set_language(ts_lang)
            .expect("Error loading the language grammar into the parser; if this error persists, consider reporting it to the library maintainers.");

        let tree = parser.parse(code.as_str(), None)?;

        let mut result = PolyglotTree {
            tree,
            code,
            working_dir: PathBuf::new(),
            language,
            node_to_subtrees_map: HashMap::new(),
        };

        let mut map = HashMap::new();
        let mut map_source: SourceMap = HashMap::new();
        let mut map_file: FileMap = HashMap::new();
        result.build_polyglot_tree(&mut map, &mut map_source, &mut map_file); // traverse the tree to build the subtrees
        result.node_to_subtrees_map = map; // set the map after its built
        Some(result)
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
        //println!("tree - passage dans la fonction principale pour créer l'arbre");
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
        //println!("tree - passage dans la fonction from directory");
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

        let mut map = HashMap::new();
        let mut map_source: SourceMap = HashMap::new();
        let mut map_file = HashMap::new();
        result.build_polyglot_tree(&mut map, &mut map_source, &mut map_file);
        result.node_to_subtrees_map = map;
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
    fn build_polyglot_tree(
        &self,
        node_tree_map: &mut HashMap<usize, PolyglotTree>,
        map_source: &mut SourceMap,
        map_file: &mut FileMap,
    ) {
        //println!("tree - start building the polyglot tree");
        let root = self.tree.root_node();
        self.build_polyglot_links(node_tree_map, root, map_source, map_file); // we get the root, and then call the recursive function
    }

    /// Internal recursive function that iterates over the nodes in the tree, and builds all subtrees as well as the polyglot link map.
    fn build_polyglot_links(
        &self,
        node_tree_map: &mut HashMap<usize, PolyglotTree>,
        node: Node,
        map_source: &mut SourceMap,
        map_file: &mut FileMap,
    ) {
        //println!("tree - passage dans la fonction polyglot links");

        //println!("AFFICHAGE DU NOEUD : ");
        //println!("{}", node.kind());

        if node.kind() == "local_variable_declaration" {
            //a parcourir
            // regarder les child
            //node.child()
            println!("INSERTION DANS LES HASHMAPS");
            println!("{}", node.named_child_count());
            println!("{}", node.child_count());
            println!("DEBUT FOR ENFANT");
            /*
            for i in 0..node.child(1).unwrap().child_count(){
                println!("i : {}", i);
                //println!("ENFANT : {}", node.child(i).unwrap().kind());
                println!("{:?}", self.node_to_code(node.child(1).unwrap().child(i).unwrap()));
            }
            */
            println!("FIN FOR ENFANT");
            for i in 0..node.named_child_count() {
                //use self.node_to_code() instead node.named_child(i) to get the child code
                println!("child : {}", node.named_child(i).unwrap().kind());
                println!("{:?}", self.node_to_code(node.child(i).unwrap()));
                if self.node_to_code(node.child(i).unwrap()) == "Source" {
                    println!("PASSAGE DANS SOURCE");
                    println!("source : {}", node.named_child(i).unwrap().kind());
                    println!("{:?}", self.node_to_code(node.child(i).unwrap()));
                    
                    let paire = node
                        .child(1)
                        .unwrap()
                        .child(2)
                        .unwrap()
                        .child(0)
                        .unwrap()
                        .child(3)
                        .unwrap();
                    println!("accès à la source : {}",self.node_to_code(node.child(1).unwrap().child(0).unwrap()));
                    println!(
                        "accès à la paire (langage, fichier): {}",
                        self.node_to_code(
                            node.child(1)
                                .unwrap()
                                .child(2)
                                .unwrap()
                                .child(0)
                                .unwrap()
                                .child(3)
                                .unwrap()
                        ),
                    );

                    println!(
                        "accès au langage : {}",
                        self.node_to_code(paire.child(1).unwrap()),
                    );
                    println!(
                        "accès au fichier : {}",
                        self.node_to_code(paire.child(3).unwrap()),
                    );

                    map_source.insert(
                        self.node_to_code(node.child(1).unwrap().child(0).unwrap()).to_string(),
                        // prendre langage dans les arguments de node
                        (
                            match util::language_string_to_enum(
                                &util::strip_quotes(self.node_to_code(paire.child(1).unwrap())),
                            ) {
                                Ok(lang) => lang,
                                Err(_) => panic!("Error: language not supported"),
                            },
                            //util::strip_quotes(self.node_to_code(paire.child(3).unwrap())).to_string(),
                            // la ligne en dessous affiche "f" mais on veut f
                            self.node_to_code(paire.child(3).unwrap()).to_string(),
                            //util::strip_quotes(self.node_to_code(paire.child(3).unwrap()))
                        ),
                    );
                }
                if self.node_to_code(node.child(i).unwrap()) == "File" {
                    println!("PASSAGE DANS FILE");
                    println!("file : {}", node.named_child(i).unwrap().kind());
                    println!("{:?}", self.node_to_code(node.child(i).unwrap()));
                    map_file.insert(
                        self.node_to_code(node.child(1).unwrap().child(0).unwrap()).to_string(),
                        self.node_to_code(node.child(1).unwrap().child(2).unwrap().child(2).unwrap().child(1).unwrap()).to_string(),
                    );
                }
            }
        }

        if self.is_polyglot_eval_call(node) {
            if !self.make_subtree(node_tree_map, map_source, map_file, node) {
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
                self.build_polyglot_links(node_tree_map, child, map_source, map_file)
            };
            if let Some(sibling) = node.next_sibling() {
                self.build_polyglot_links(node_tree_map, sibling, map_source, map_file)
            };
        }
    }

    fn get_polyglot_call_python(&self, node: Node) -> Option<&str> {
        //println!("tree - passage dans la fonction polyglot python");
        let child = node.child(0)?;
        if node.kind().eq("call") && child.kind().eq("attribute") {
            return Some(self.node_to_code(child));
        }
        None
    }

    fn get_polyglot_call_js(&self, node: Node) -> Option<&str> {
        //println!("tree - passage dans la fonction polyglot js");
        let child = node.child(0)?;
        if node.kind().eq("call_expression") && child.kind().eq("member_expression") {
            return Some(self.node_to_code(child));
        }
        None
    }

    fn get_polyglot_call_java(&self, node: Node) -> Option<&str> {
        //println!("tree - passage dans la fonction polyglot java");
        let child = node.child(2)?;
        if node.kind().eq("method_invocation") && child.kind().eq("identifier") {
            return Some(self.node_to_code(child));
        }
        None
    }

    fn is_polyglot_eval_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => {
                matches!(self.get_polyglot_call_python(node), Some("polyglot.eval"))
            }
            Language::JavaScript => matches!(
                self.get_polyglot_call_js(node),
                Some("Polyglot.eval") | Some("Polyglot.evalFile")
            ),
            Language::Java => matches!(self.get_polyglot_call_java(node), Some("eval")),
        }
    }

    fn is_polyglot_import_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => matches!(
                self.get_polyglot_call_python(node),
                Some("polyglot.import_value")
            ),
            Language::JavaScript => {
                matches!(self.get_polyglot_call_js(node), Some("Polyglot.import"))
            }
            Language::Java => matches!(self.get_polyglot_call_java(node), Some("getMember")),
        }
    }

    fn is_polyglot_export_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => matches!(
                self.get_polyglot_call_python(node),
                Some("polyglot.export_value")
            ),
            Language::JavaScript => {
                matches!(self.get_polyglot_call_js(node), Some("Polyglot.export"))
            }
            Language::Java => matches!(self.get_polyglot_call_java(node), Some("putMember")),
        }
    }

    fn make_subtree(
        &self,
        node_tree_map: &mut HashMap<usize, PolyglotTree>,
        map_source: &mut SourceMap,
        map_file: &mut FileMap,
        node: Node,
    ) -> bool {
        println!("tree - création d'un nouveau sous arbre");
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

    fn make_subtree_python(&self, node: &Node) -> Option<PolyglotTree> {
        println!("tree - création sous arbre python");
        let arg1 = node.child(1)?.child(1)?.child(0)?;
        let arg2 = node.child(1)?.child(3)?.child(0)?;

        let mut new_code: Option<String> = None;
        let mut new_lang: Option<String> = None;
        let mut path: Option<PathBuf> = None;

        // Python polyglot calls use a single function and differentiate by argument names, which are mandatory.
        // We need to check both arguments for each possible case, and then check again at the end we have enough information.
        match self.node_to_code(arg1) {
            "path" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg1.next_sibling()?.next_sibling()?));
                path = Some(self.working_dir.clone());
                let new_path = match PathBuf::from_str(tmp.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp.as_str()
                        );
                        return None;
                    }
                };
                path = path.map(|mut p| {
                    p.push(new_path);
                    p
                });
            }

            "language" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg1.next_sibling()?.next_sibling()?));
                new_lang = Some(String::from(tmp.as_str()));
            }

            "string" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg1.next_sibling()?.next_sibling()?));
                new_code = Some(String::from(tmp.as_str()));
            }
            other => {
                eprintln!(
                    "Warning: unable to handle polyglot call argument {other} at position {}",
                    arg1.start_position()
                );
                return None;
            }
        }

        match self.node_to_code(arg2) {
            "path" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg2.next_sibling()?.next_sibling()?));
                path = Some(self.working_dir.clone());
                let new_path = match PathBuf::from_str(tmp.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp.as_str()
                        );
                        return None;
                    }
                };
                path = path.map(|mut p| {
                    p.push(new_path);
                    p
                });
            }

            "language" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg2.next_sibling()?.next_sibling()?));
                new_lang = Some(String::from(tmp.as_str()));
            }

            "string" => {
                let tmp =
                    util::strip_quotes(self.node_to_code(arg2.next_sibling()?.next_sibling()?));

                new_code = Some(String::from(tmp.as_str()));
            }

            other => {
                eprintln!(
                    "Warning: unable to handle polyglot call argument {other} at position {}",
                    arg2.start_position()
                );
                return None;
            }
        }

        // We convert the language, if there was one
        let new_lang = match new_lang {
            Some(s) => match util::language_string_to_enum(s.as_str()) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}");
                    return None;
                }
            },
            None => {
                eprintln!(
                    "Warning: no language argument provided for polyglot call at position {}",
                    node.start_position()
                );
                return None;
            }
        };

        let subtree = match new_code {
            Some(c) => Self::from_directory(c, new_lang, self.working_dir.clone())?,
            None => Self::from_path(
                // No raw code, check for a path
                match path {
                    Some(p) => p,
                    None => {
                        // No path either -> we cant build the tree
                        eprintln!("Warning:: no path or string argument provided to Python polyglot call at position {}", node.start_position());
                        return None;
                    }
                },
                new_lang,
            )?,
        };
        Some(subtree)
    }

    fn make_subtree_js(&self, node: &Node) -> Option<PolyglotTree> {
        println!("tree - création sous arbre js");
        let call_type = node.child(0)?.child(2)?; // function name
        let arg1 = node.child(1)?.child(1)?; // language
        let arg2 = node.child(1)?.child(3)?; // code

        // JavaScript uses a different function for evaluating raw code and files, so we have two cases
        match self.node_to_code(call_type) {
            "eval" => {
                // Arguments are positional, and always at the same spot
                let tmp_lang = util::strip_quotes(self.node_to_code(arg1));
                let tmp_code = util::strip_quotes(self.node_to_code(arg2));

                let new_lang = match util::language_string_to_enum(tmp_lang.as_str()) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!(
                            "Could not convert argument {} to language due to error: {e}",
                            tmp_lang.as_str()
                        );
                        return None;
                    }
                };

                let new_code = String::from(tmp_code.as_str());
                Self::from_directory(new_code, new_lang, self.working_dir.clone())
            }

            "evalFile" => {
                let tmp_lang = util::strip_quotes(self.node_to_code(arg1));

                let new_lang = match util::language_string_to_enum(tmp_lang.as_str()) {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!(
                            "Could not convert argument {} to language due to error: {e}",
                            tmp_lang.as_str()
                        );
                        return None;
                    }
                };

                let tmp_path = util::strip_quotes(self.node_to_code(arg2));

                let mut path = self.working_dir.clone();

                let new_path = match PathBuf::from_str(tmp_path.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp_path.as_str()
                        );
                        return None;
                    }
                };

                path.push(new_path);

                Self::from_path(path, new_lang)
            }

            other => {
                eprintln!(
                    "Warning: unable to identify polyglot function call {other} at position {}",
                    node.start_position()
                );
                None
            }
        }
    }

    fn make_subtree_java(
        &self,
        node: &Node,
        map_source: &mut SourceMap,
        map_file: &mut FileMap,
    ) -> Option<PolyglotTree> {
        println!("tree - création sous arbre java");
        // Java uses positional arguments, so they will always be accessible with the same route.

        println!("MAP SOURCE : {:?}", map_source);
        println!("MAP FILE : {:?}", map_file);

        // look at the number of children to count the args
        let nb_args = node.child_count() - 1;
        println!("nb_args : {}", nb_args);
        for i in 0..nb_args {
            println!("i : {}", i);
            let arg = node.child(i + 1)?;
            println!("arg : {}", self.node_to_code(arg));
        }

        //si node.child(3) a un enfant, c'est que c'est un code, si node.child(3) a 2 enfants: le premier est le language, le deuxième est le code
        let nb_child: usize = node.child(3)?.child_count();
        println!("nb_child : {}", nb_child);
        for i in 0..nb_child {
            println!("i : {}", i);
            let arg = node.child(3)?.child(i)?;
            println!("arg : {}", self.node_to_code(arg));
        }

        if nb_child == 3 {
            println!("nb child : {}", nb_child);
            let arg1 = node.child(3)?.child(1)?; // code
            println!("code : {}", self.node_to_code(arg1));

            //getting source
            let source = map_source.get(self.node_to_code(arg1));
            //dbg!(source);
            println!("source : {:?}", source.expect("source not found"));

            //guetting language
            let tmp_language = &source.expect("").0;

            //getting file
            let file: Option<&String> = map_file.get(&source.expect("source not found").1);
            println!("file : {}", file.expect("file not found"));

            //open the file of the path
            let mut openfile = File::open(file.expect("file not found")).unwrap();

            //putting file code in a string
            let mut code = String::new();
            openfile.read_to_string(&mut code).unwrap();

            //return option<polyglottree> type
            Self::from_directory(code, tmp_language.clone(), self.working_dir.clone());
        } else if nb_child == 5 {
            println!("nb child : {}", nb_child);
            let r = retreive_invocation_arguments(node)?;
            //let arg1 = r[0]; // language
            //let arg2 = r[1]; // code
            let r = ArgsIt::new(node);
            /*
            for arg in r {
                println!("arg : {:#?}", arg);
            }
            */
            let arg1 = node.child(3)?.child(1)?; // language
            println!("ENDROIT QUI BLOQUE");
            println!("arg1 : {:?}", self.node_to_code(arg1));
            let arg2 = node.child(3)?.child(3)?; // code
            
            println!("language : {}", self.node_to_code(arg1));
            println!("code : {}", self.node_to_code(arg2));

            let s = util::strip_quotes(self.node_to_code(arg1));
            //ajout de strip_quotes car "\"python\""
            let new_lang = match util::language_string_to_enum(&util::strip_quotes(&s)) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Could not convert argument {s} to language due to error: {e}",);
                    return None;
                }
            };

            let new_code = util::strip_quotes(self.node_to_code(arg2));
            Self::from_directory(new_code, new_lang, self.working_dir.clone());
        }
        println!("fin");
        return None;
    }
}

fn retreive_invocation_arguments<'a>(node: &Node<'a>) -> Option<Vec<Node<'a>>> {
    let mut walk = node.child_by_field_name("arguments")?.walk();
    let mut r = vec![];
    assert!(walk.goto_first_child());
    loop {
        let node = walk.node();
        if !node.kind().contains("[,()]") {
            r.push(node);
        }
        if !walk.goto_next_sibling() {
            break;
        }
    }
    Some(r)
}

struct ArgsIt<'a> {
    walk: tree_sitter::TreeCursor<'a>,
}

impl<'a> ArgsIt<'a> {
    fn new(node: &'a Node) -> Self {
        let mut walk = node.walk();
        walk.goto_first_child();
        Self { walk }
    }
}

impl<'a> Iterator for ArgsIt<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.walk.node();
            if !node.kind().contains("[,()]") {
                return Some(node);
            }
            if !self.walk.goto_next_sibling() {
                return None;
            }
        }
    }
}
