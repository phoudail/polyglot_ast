use super::util;
use super::util::Language;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{any::Any, str::FromStr};
use tree_sitter::{Node, Parser, Tree};

mod polyglot_zipper;

pub struct PolyglotTree {
    tree: Tree,
    code: String,
    working_dir: PathBuf,
    language: Language,
    node_to_subtrees_map: HashMap<Node, PolyglotTree>,
}

impl PolyglotTree {
    pub fn from(code: &str, language: &str) -> Result<PolyglotTree, util::InvalidArgumentError> {
        let mut parser = Parser::new();
        let lang = util::language_string_to_treesitter(language)?;

        parser
            .set_language(lang)
            .expect("Error loading the language grammar into the parser.");

        let tree = parser
            .parse(code, None)
            .expect("Error parsing the language code.");

        let mut result = PolyglotTree {
            tree,
            code: String::from(code),
            working_dir: PathBuf::new(),
            language: util::language_string_to_enum(language)?,
            node_to_subtrees_map: HashMap::new(),
        };

        let mut map = HashMap::new();
        result.build_polyglot_tree(&mut map);
        result.node_to_subtrees_map = map;
        Ok(result)
    }

    pub fn node_to_code(&self, node: Node) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }

    pub fn root_node(&self) -> Node {
        self.tree.root_node()
    }

    fn build_polyglot_tree(&self, node_tree_map: &mut HashMap<Node, PolyglotTree>) {
        let root = self.tree.root_node();
        self.build_polyglot_links(node_tree_map, root);
    }

    fn build_polyglot_links(&self, node_tree_map: &mut HashMap<Node, PolyglotTree>, node: Node) {
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
        node_tree_map: &mut HashMap<Node, PolyglotTree>,
        node: Node,
    ) -> Option<()> {
        let mut new_code: Option<&str> = None;
        let mut new_lang: Option<&str> = None;
        match self.language {
            Language::Python => {
                let arg1 = node.child(1)?.child(1)?;
                let arg2 = node.child(1)?.child(3)?;

                match self.node_to_code(arg1.child(0)?) {
                    "path" => {
                        let mut tmp = self
                            .node_to_code(arg1.next_sibling()?.next_sibling()?)
                            .chars();
                        tmp.next();
                        tmp.next_back();
                        let tmp = PathBuf::from_str(tmp.as_str());
                        todo!()
                    }
                    "language" => {
                        let mut tmp = self
                            .node_to_code(arg1.next_sibling()?.next_sibling()?)
                            .chars();
                        tmp.next();
                        tmp.next_back();
                        new_lang = Some(tmp.as_str());
                    }
                    "string" => {
                        let mut tmp = self
                            .node_to_code(arg1.next_sibling()?.next_sibling()?)
                            .chars();
                        tmp.next();
                        tmp.next_back();
                        new_code = Some(tmp.as_str());
                    }
                    other => {
                        eprintln!("Warning: unable to handle polyglot call argument {other} at position {}", node.start_position());
                        return None;
                    }
                }
                println!("{}\n{}", self.node_to_code(arg1), self.node_to_code(arg2));
            }
            Language::JavaScript => todo!(),
            Language::Java => todo!(),
        }

        let subtree = match PolyglotTree::from(new_code?, new_lang?) {
            Ok(t) => t,
            Err(_) => return None,
        };
        node_tree_map.insert(node, subtree);
        Some(())
    }
}
