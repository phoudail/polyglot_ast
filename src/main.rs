use std::path::PathBuf;

use polyglot_ast::{Language, PolyglotTree, TreePrinter};

fn main() {
    let file = PathBuf::from("TestSamples/JavaTest3.java");
    let tree =
        PolyglotTree::from_path(file, Language::Java).expect("Should not have parsing issues");
    let mut tp = TreePrinter::new();
    tree.apply(&mut tp);
    println!("{}", tp.get_result())
}
