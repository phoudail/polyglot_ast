use std::collections::HashMap;
use tree_sitter::Parser;

use crate::util::InvalidArgumentError;

use super::util;

struct PolyglotTree<'a> {
    tree: tree_sitter::Tree,
    code: &'a str,
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
            code,
            language: lang,
            node_to_subtrees_map: HashMap::new(),
        })
    }

    pub fn node_to_code(&self, node: tree_sitter::Node<'a>) -> &'a str {
        &self.code[node.start_byte()..node.end_byte()]
    }    
}
