use tree_sitter::{Node, TreeCursor};

use super::util::{InvalidArgumentError, Language};

use super::PolyglotTree;

pub struct PolyglotZipper<'a> {
    tree: &'a PolyglotTree,
    node: TreeCursor<'a>,
}

impl PolyglotZipper<'_> {
    pub fn from<'a>(tree: &'a PolyglotTree) -> PolyglotZipper<'a> {
        Self::_from(tree, tree.root_node())
    }

    fn _from<'a>(tree: &'a PolyglotTree, node: Node<'a>) -> PolyglotZipper<'a> {
        PolyglotZipper {
            tree,
            node: node.walk(),
        }
    }

    fn node(&self) -> Node {
        self.node.node()
    }

    pub fn is_polyglot_eval_call(&self) -> bool {
        self.tree.is_polyglot_eval_call(self.node())
    }

    pub fn is_polyglot_import_call(&self) -> bool {
        self.tree.is_polyglot_import_call(self.node())
    }

    pub fn is_polyglot_export_call(&self) -> bool {
        self.tree.is_polyglot_export_call(self.node())
    }

    pub fn kind(&self) -> &str {
        if self.is_polyglot_eval_call() {
            return "polyglot_eval_call";
        } else if self.is_polyglot_import_call() {
            return "polyglot_import_call";
        } else if self.is_polyglot_export_call() {
            return "polyglot_export_call";
        }
        self.node().kind()
    }

    pub fn code(&self) -> &str {
        self.tree.node_to_code(self.node())
    }

    pub fn start_position(&self) -> tree_sitter::Point {
        self.node().start_position()
    }

    pub fn end_position(&self) -> tree_sitter::Point {
        self.node().end_position()
    }

    pub fn get_binding_name(&self) -> Result<String, InvalidArgumentError> {
        if self.is_polyglot_import_call() || self.is_polyglot_export_call() {
            return match self.get_lang() {
                Language::Python => match self.get_python_binding() {
                    Some(s) => Ok(s),
                    None => Err(InvalidArgumentError), // todo: make this into a proper error enum
                },
                Language::JavaScript => todo!(),
                Language::Java => todo!(),
            };
        }
        Err(InvalidArgumentError)
    }

    fn get_python_binding(&self) -> Option<String> {
        Some(String::from(self.child(1)?.child(1)?.code()))
    }

    pub fn get_lang(&self) -> &Language {
        &self.tree.language
    }

    pub fn goto_first_child(&mut self) -> bool {
        let my_id = self.node().id();
        let subtree = self.tree.node_to_subtrees_map.get(&my_id);

        match subtree {
            Some(t) => {
                self.tree = t;
                self.node = t.root_node().walk();
                true
            }

            None => self.node.goto_first_child(),
        }
    }

    pub fn goto_next_sibling(&mut self) -> bool {
        self.node.goto_next_sibling()
    }

    pub fn child(&self, i: usize) -> Option<PolyglotZipper> {
        let my_id = self.node().id();
        let subtree = self.tree.node_to_subtrees_map.get(&my_id);

        match subtree {
            Some(t) => Some(PolyglotZipper {
                tree: t,
                node: t.root_node().walk(),
            }),

            None => Some(Self::_from(self.tree, self.node.node().child(i)?)),
        }
    }

    pub fn next_sibling(&self) -> Option<PolyglotZipper> {
        Some(Self::_from(self.tree, self.node().next_sibling()?))
    }

    pub fn prev_sibling(&self) -> Option<PolyglotZipper> {
        Some(Self::_from(self.tree, self.node().prev_sibling()?))
    }
}
