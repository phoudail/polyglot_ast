use std::collections::HashMap;
use tree_sitter::Parser;

use crate::util::InvalidArgumentError;

use super::util;

mod polyglot_zipper;

use polyglot_zipper::*;

struct PolyglotTree<'a> {
    tree: tree_sitter::Tree,
    code: String,
    language: tree_sitter::Language,
    node_to_subtrees_map: HashMap<tree_sitter::Node<'a>, PolyglotTree<'a>>,
}

impl<'a> PolyglotTree<'a> {


    pub fn from(code: &'a str, language: &str) -> Result<PolyglotTree<'a>, InvalidArgumentError> {
        let mut parser = Parser::new();
        let lang = util::language_string_to_treesitter(language)?;

        parser.set_language(lang).expect("Error loading the language grammar into the parser.");
        
        let tree = parser.parse(code, None).expect("Error parsing the language code.");
        
        Ok(PolyglotTree {
            tree,
            code: String::from(code),
            language: lang,
            node_to_subtrees_map: HashMap::new(),
        }.build_polyglot_tree())
    }

    pub fn node_to_code(&self, node: tree_sitter::Node<'a>) -> &str {
        &self.code[node.start_byte()..node.end_byte()]
    }

    fn build_polyglot_tree(self) -> PolyglotTree<'a> {
        let root = self.tree.root_node();
        self.build_polyglot_links(root);
        self
    }

    fn build_polyglot_links(&self, node: tree_sitter::Node) {
        if is_polyglot_call(node) {
            self.make_subtree(node);
        } else {
            match node.child(0) {
                Some(child) => self.build_polyglot_links(child),
                None => ()
            }
            match node.next_sibling() {
                Some(sibling) => self.build_polyglot_links(sibling),
                None => ()
            }
        }
    }

    fn make_subtree(&self, node: tree_sitter::Node) {
        //TODO
    }

}
