use polyglot_ast::polyglot_tree::PolyglotTree;

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
    let tree = PolyglotTree::from("x = 42\npolyglot.eval(path=\"file\", language=x)", "python").expect("Python assignment should be properly parsed");
    println!("{}", tree.node_to_code(tree.root_node()))
}
