pub(crate) struct TreeSitterCST<'tree, 'text> {
    pub cst: &'tree tree_sitter::Tree,
    source: &'text str,
}

impl<'tree, 'text> TreeSitterCST<'tree, 'text> {
    pub fn node_to_code(&self, node: &tree_sitter::Node<'tree>) -> &'text str {
        &self.source[node.start_byte()..node.end_byte()]
    }
}

trait Node<'a> {
    fn child(&self, i: usize) -> Self;
}

pub(crate) fn into<'tree, 'text>(
    tree: &'tree Option<tree_sitter::Tree>,
    file_content: &'text str,
) -> TreeSitterCST<'tree, 'text> {
    let cst = TreeSitterCST {
        cst: tree.as_ref().expect("tree"),
        source: file_content,
    };
    cst
}

pub(crate) fn parse(file_content: &String) -> Option<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    let ts_lang = crate::util::language_enum_to_treesitter(&crate::Language::Java);

    parser
        .set_language(ts_lang)
        .expect("Error loading the language grammar into the parser; if this error persists, consider reporting it to the library maintainers.");

    let tree = parser.parse(file_content.as_str(), None);
    tree
}
