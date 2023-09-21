//! A simple example AST of:
//! ```python
//! print('hello')
//! ```

use polyglot_ast::prelude::*;

fn main() {
    let code = "print('hello')".into();
    let lang = Language::Python;
    let parsed = PolyglotTree::parse(code, lang);

    let tree = parsed.tree().as_ref().unwrap();
    let zipper = PolyglotZipper::new(tree);

    let tree_printer = &mut TreePrinter::new();
    tree_printer.process_impl(zipper).unwrap();

    let result = tree_printer.get_result();
    println!("{}", result);
}
