use polyglot_ast::polyglot_tree::PolyglotTree;

fn main() {
    let tree = PolyglotTree::from(
        "x = 42\npolyglot.eval(path=\"GraalSamples/test_b.py\", language=\"python\")",
        polyglot_ast::util::Language::Python,
    )
    .expect("Should not have parsing issues");
    let mut tp = polyglot_ast::polyglot_tree::polyglot_processor::TreePrinter::new();
    tree.apply(&mut tp);
    println!("{}", tp.get_result())
}
