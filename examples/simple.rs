//! A simple example building and using the Polyglote AST.

use polyglot_ast::prelude::*;

fn main() {
    dbg!(std::env::current_dir().unwrap());
    let path = "TestSamples/JavaTest.java";
    let polyglote_ast = PolyglotAstBuilder::set_entry_point(path)
        .set_entry_lang(Language::Java)
        .build()
        .unwrap();

    let tree_printer = &mut PolyglotTreeSerializer::from(&polyglote_ast);
    println!("{}", tree_printer);
}
