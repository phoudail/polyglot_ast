use super::util;
use super::util::Language;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use tree_sitter::{Node, Parser, Tree};

mod polyglot_zipper;
use polyglot_zipper::PolyglotZipper;

pub struct PolyglotTree {
    tree: Tree,
    code: String,
    working_dir: PathBuf,
    language: Language,
    node_to_subtrees_map: HashMap<usize, PolyglotTree>,
}

impl PolyglotTree {
    pub fn from(
        code: impl ToString,
        language: impl ToString,
    ) -> Result<PolyglotTree, util::InvalidArgumentError> {
        let code = code.to_string();
        let language = language.to_string();

        let mut parser = Parser::new();
        let lang = util::language_string_to_treesitter(language.as_str())?;

        parser
            .set_language(lang)
            .expect("Error loading the language grammar into the parser.");

        let tree = parser
            .parse(code.as_str(), None)
            .expect("Error parsing the language code.");

        let mut result = PolyglotTree {
            tree,
            code,
            working_dir: PathBuf::new(),
            language: util::language_string_to_enum(language.as_str())?,
            node_to_subtrees_map: HashMap::new(),
        };

        let mut map = HashMap::new();
        result.build_polyglot_tree(&mut map);
        result.node_to_subtrees_map = map;
        Ok(result)
    }

    pub fn root_zipper(&self) -> polyglot_zipper::PolyglotZipper {
        PolyglotZipper::from(self, self.root_node())
    }

    pub fn node_to_code(&self, node: Node) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }

    pub fn root_node(&self) -> Node {
        self.tree.root_node()
    }

    fn build_polyglot_tree(&self, node_tree_map: &mut HashMap<usize, PolyglotTree>) {
        let root = self.tree.root_node();
        self.build_polyglot_links(node_tree_map, root);
    }

    fn build_polyglot_links(&self, node_tree_map: &mut HashMap<usize, PolyglotTree>, node: Node) {
        if self.is_polyglot_call(node) {
            if self.make_subtree(node_tree_map, node).is_none() {
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

    pub fn is_polyglot_call(&self, node: Node) -> bool {
        match self.language {
            Language::Python => {
                node.kind().eq("call")
                    && node
                        .child(0)
                        .map(|child| {
                            child.kind().eq("attribute")
                                && self.node_to_code(child).eq("polyglot.eval")
                        })
                        .unwrap_or(false)
            }
            Language::JavaScript => todo!(),
            Language::Java => todo!(),
        }
    }

    fn make_subtree(
        &self,
        node_tree_map: &mut HashMap<usize, PolyglotTree>,
        node: Node,
    ) -> Option<()> {
        let new_code: String;
        let new_lang: String;
        match self.language {
            Language::Python => match self.handle_python_args(&node) {
                Some((s1, s2)) => {
                    new_code = s1;
                    new_lang = s2;
                }
                None => return None,
            },
            Language::JavaScript => match self.handle_js_args(&node) {
                Some((s1, s2)) => {
                    new_code = s1;
                    new_lang = s2;
                }
                None => return None,
            },
            Language::Java => match self.handle_java_args(&node) {
                Some((s1, s2)) => {
                    new_code = s1;
                    new_lang = s2;
                }
                None => return None,
            },
        }

        let subtree = match PolyglotTree::from(new_code, new_lang) {
            Ok(t) => t,
            Err(_) => return None,
        };
        node_tree_map.insert(node.id(), subtree);
        Some(())
    }

    fn handle_js_args(&self, _node: &Node) -> Option<(String, String)> {
        todo!()
    }

    fn handle_java_args(&self, _node: &Node) -> Option<(String, String)> {
        todo!()
    }

    fn handle_python_args(&self, node: &Node) -> Option<(String, String)> {
        let arg1 = node.child(1)?.child(1)?.child(0)?;
        let arg2 = node.child(1)?.child(3)?.child(0)?;

        let mut new_code: Option<String> = None;
        let mut new_lang: Option<String> = None;

        match self.node_to_code(arg1) {
            "path" => {
                let mut tmp = self
                    .node_to_code(arg1.next_sibling()?.next_sibling()?)
                    .chars();
                tmp.next();
                tmp.next_back();
                let mut path = self.working_dir.clone();
                let new_path = match PathBuf::from_str(tmp.as_str()) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!(
                                    "Warning: could not build subtree for {} because of the following error: {e}",
                                    tmp.as_str()
                                );
                        return None;
                    }
                };
                path.push(new_path);
                new_code = Some(match std::fs::read_to_string(path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp.as_str()
                        );
                        return None;
                    }
                });
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
                let mut path = self.working_dir.clone();
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
                path.push(new_path);
                new_code = Some(match std::fs::read_to_string(path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!(
                            "Warning: could not build subtree for {} because of error {e}",
                            tmp.as_str()
                        );
                        return None;
                    }
                });
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
        let new_code = match new_code {
            Some(s) => s,
            None => {
                eprintln!("Warning: no code (as \"path\" or \"string\") argument provided for polyglot call at position {}", node.start_position());
                return None;
            }
        };
        let new_lang = match new_lang {
            Some(s) => s,
            None => {
                eprintln!(
                    "Warning: no language argument provided for polyglot call at position {}",
                    node.start_position()
                );
                return None;
            }
        };
        Some((new_code, new_lang))
    }
}
