trait TextProvider<'text> {
    type I: Iterator<Item = &'text [u8]> + 'text;
    type N<'t>;
    fn text(&mut self, node: &Self::N<'_>) -> Self::I;
}

#[derive(Debug)]

pub(crate) struct TreeSitterCST<'tree, 'text> {
    pub cst: &'tree tree_sitter::Tree,
    source: &'text str,
}

impl<'tree, 'text> TreeSitterCST<'tree, 'text> {
    pub fn node_to_code(&self, node: &tree_sitter::Node<'tree>) -> &'text str {
        &self.source[node.start_byte()..node.end_byte()]
    }
}

impl<'tree, 'text> tree_sitter::TextProvider<'text> for &TreeSitterCST<'tree, 'text> {
    type I = std::iter::Once<&'text [u8]>;

    fn text(&mut self, node: tree_sitter::Node) -> Self::I {
        std::iter::once(self.node_to_code(&node).as_bytes())
    }
}

impl<'tree, 'text> TextProvider<'text> for &TreeSitterCST<'tree, 'text> {
    type I = <Self as tree_sitter::TextProvider<'text>>::I;
    type N<'t> = tree_sitter::Node<'t>;
    fn text(&mut self, node: &Self::N<'_>) -> Self::I {
        std::iter::once(self.node_to_code(node).as_bytes())
    }
}

pub(crate) fn into<'tree, 'text>(
    tree: Option<&'tree tree_sitter::Tree>,
    file_content: &'text str,
) -> TreeSitterCST<'tree, 'text> {
    let cst = TreeSitterCST {
        cst: tree.as_ref().expect("tree"),
        source: file_content,
    };
    cst
}

pub(crate) struct TreeSitterCstArcStr {
    cst: tree_sitter::Tree,
    source: std::sync::Arc<str>,
}

impl TreeSitterCstArcStr {
    pub(crate) fn new(cst: tree_sitter::Tree, source: std::sync::Arc<str>) -> Self {
        Self { cst, source }
    }
    pub fn node_to_code<'text>(&'text self, node: &tree_sitter::Node) -> &'text str {
        &self.source[node.start_byte()..node.end_byte()]
    }
}

impl<'text> tree_sitter::TextProvider<'text> for &'text TreeSitterCstArcStr {
    type I = std::iter::Once<&'text [u8]>;

    fn text(&mut self, node: tree_sitter::Node) -> Self::I {
        std::iter::once(self.source[node.start_byte()..node.end_byte()].as_bytes())
    }
}

impl<'text> TextProvider<'text> for &'text TreeSitterCstArcStr {
    type I = <Self as tree_sitter::TextProvider<'text>>::I;
    type N<'t> = tree_sitter::Node<'t>;
    fn text(&mut self, node: &Self::N<'_>) -> Self::I {
        std::iter::once(self.node_to_code(node).as_bytes())
    }
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

trait Node<'a> {
    fn child(&self, i: usize) -> Self;
}
