use super::util;
use super::util::Language;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use tree_sitter::{Node, Parser, Tree};

pub mod polyglot_processor;
pub mod polyglot_zipper;

pub struct PolyglotTree {
    tree: Tree,
    code: String,
    working_dir: PathBuf,
    language: Language,
    node_to_subtrees_map: HashMap<usize, PolyglotTree>,
}

impl PolyglotTree {
    pub fn from(code: impl ToString, language: Language) -> Option<PolyglotTree> {
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
            working_dir: PathBuf::new(),
            language,
            node_to_subtrees_map: HashMap::new(),
        };

        let mut map = HashMap::new();
        result.build_polyglot_tree(&mut map);
        result.node_to_subtrees_map = map;
        Some(result)
    }

    pub fn from_directory(
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

        let mut map = HashMap::new();
        result.build_polyglot_tree(&mut map);
        result.node_to_subtrees_map = map;
        Some(result)
    }

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
        result.build_polyglot_tree(&mut map);
        result.node_to_subtrees_map = map;
        Some(result)
    }

    pub fn apply(&self, processor: &mut impl polyglot_processor::PolygotProcessor) {
        processor.process(polyglot_zipper::PolyglotZipper::from(self))
    }

    fn node_to_code(&self, node: Node) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }

    fn root_node(&self) -> Node {
        self.tree.root_node()
    }

    fn build_polyglot_tree(&self, node_tree_map: &mut HashMap<usize, PolyglotTree>) {
        let root = self.tree.root_node();
        self.build_polyglot_links(node_tree_map, root);
    }

    fn build_polyglot_links(&self, node_tree_map: &mut HashMap<usize, PolyglotTree>, node: Node) {
        if self.is_polyglot_eval_call(node) {
            if !self.make_subtree(node_tree_map, node) {
                eprintln!(
                    "Warning: unable to make subtree for polyglot call at position {}",
                    node.start_position()
                )
            }
        } else {
            match node.child(0) {
                Some(child) => self.build_polyglot_links(node_tree_map, child),
                None => (),
            };
            match node.next_sibling() {
                Some(sibling) => self.build_polyglot_links(node_tree_map, sibling),
                None => (),
            };
        }
    }

    fn get_polyglot_call_python(&self, node: Node) -> Option<&str> {
        let child = node.child(0)?;
        if node.kind().eq("call") && child.kind().eq("attribute") {
            return Some(self.node_to_code(child));
        } else {
            return None;
        }
    }

    fn get_polyglot_call_js(&self, node: Node) -> Option<&str> {
        let child = node.child(0)?;
        if node.kind().eq("call_expression") && child.kind().eq("member_expression") {
            return Some(self.node_to_code(child));
        } else {
            return None;
        }
    }

    fn is_polyglot_eval_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => match self.get_polyglot_call_python(node) {
                Some("polyglot.eval") => true,
                _ => false,
            },
            Language::JavaScript => match self.get_polyglot_call_js(node) {
                Some("Polyglot.eval") | Some("Polyglot.evalFile") => true,
                _ => false,
            },
            Language::Java => todo!(),
        }
    }

    fn is_polyglot_import_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => match self.get_polyglot_call_python(node) {
                Some("polyglot.import_value") => true,
                _ => false,
            },
            Language::JavaScript => match self.get_polyglot_call_js(node) {
                Some("Polyglot.import") => true,
                _ => false,
            },
            Language::Java => todo!(),
        }
    }

    fn is_polyglot_export_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => match self.get_polyglot_call_python(node) {
                Some("polyglot.export_value") => true,
                _ => false,
            },
            Language::JavaScript => match self.get_polyglot_call_js(node) {
                Some("Polyglot.export") => true,
                _ => false,
            },
            Language::Java => todo!(),
        }
    }

    fn make_subtree(&self, node_tree_map: &mut HashMap<usize, PolyglotTree>, node: Node) -> bool {
        let subtree: PolyglotTree;
        let result: Option<PolyglotTree> = match self.language {
            Language::Python => self.make_subtree_python(&node),
            Language::JavaScript => self.make_subtree_js(&node),
            Language::Java => self.make_subtree_java(&node),
        };

        subtree = match result {
            Some(t) => t,
            None => return false,
        };

        node_tree_map.insert(node.id(), subtree);

        true // signale everything went right
    }

    fn make_subtree_python(&self, node: &Node) -> Option<PolyglotTree> {
        let arg1 = node.child(1)?.child(1)?.child(0)?;
        let arg2 = node.child(1)?.child(3)?.child(0)?;

        let mut new_code: Option<String> = None;
        let mut new_lang: Option<String> = None;
        let mut path: Option<PathBuf> = None;

        match self.node_to_code(arg1) {
            "path" => {
                let mut tmp = self
                    .node_to_code(arg1.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
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
                path = path.map(|mut p| {p.push(new_path); p} );
            }

            "language" => {
                let mut tmp = self
                    .node_to_code(arg1.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
                new_lang = Some(String::from(tmp.as_str()));
            }

            "string" => {
                let mut tmp = self
                    .node_to_code(arg1.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
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
                let mut tmp = self
                    .node_to_code(arg2.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
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
                path = path.map(|mut p| {p.push(new_path); p} );
            }

            "language" => {
                let mut tmp = self
                    .node_to_code(arg2.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
                new_lang = Some(String::from(tmp.as_str()));
            }

            "string" => {
                let mut tmp = self
                    .node_to_code(arg2.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
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
                match path {
                    Some(p) => p,
                    None => {
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
        let call_type = node.child(0)?.child(2)?;
        let arg1 = node.child(1)?.child(1)?;
        let arg2 = node.child(1)?.child(3)?;

        match self.node_to_code(call_type) {
            "eval" => {
                let mut tmp_lang = self.node_to_code(arg1).chars(); // JS uses positional arguments,
                let mut tmp_code = self.node_to_code(arg2).chars(); // no need to check

                tmp_lang.next();
                tmp_lang.next_back();
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

                tmp_code.next();
                tmp_code.next_back();
                let new_code = String::from(tmp_code.as_str());
                Self::from_directory(new_code, new_lang, self.working_dir.clone())
            }

            "evalFile" => {
                let mut tmp_lang = self.node_to_code(arg1).chars();

                tmp_lang.next();
                tmp_lang.next_back();
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

                let mut tmp_path = self.node_to_code(arg2).chars();
                tmp_path.next();
                tmp_path.next_back();

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
                return None;
            }
        }
    }

    fn make_subtree_java(&self, _node: &Node) -> Option<PolyglotTree> {
        todo!()
    }
}
