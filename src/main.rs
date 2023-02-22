use polyglot_ast::polyglot_tree::{PolyglotTree, polyglot_zipper};

fn main() {
    // use tree_sitter::Parser;
    //     let code = r#"
    //     function double(x) {
    //         return x * 2;
    //     }
    // "#;
    //     let mut parser = Parser::new();
    //     parser
    //         .set_language(tree_sitter_javascript::language())
    //         .expect("Error loading JavaScript grammar");
    //     let parsed = parser.parse(code, None);
    //     println!("{:#?}", parsed);

    //     let code = r#"
    //         def double(x):
    //             return x * 2
    //             "#;
    //     let mut parser = Parser::new();
    //     parser
    //         .set_language(tree_sitter_python::language())
    //         .expect("Error loading Python grammar");
    //     let parsed = parser.parse(code, None);
    //     println!("{:#?}", parsed);
    let tree = PolyglotTree::from(
        "x = 42\npolyglot.eval(path=\"GraalSamples/test_b.py\", language=\"python\")",
        polyglot_ast::util::Language::Python,
    ).expect("Should not have parsing issues");
    let mut tp = polyglot_ast::polyglot_tree::polyglot_processor::TreePrinter::new();
    tree.apply(&mut tp);
    println!("{}", tp.get_result())
}
