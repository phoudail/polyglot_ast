use std::path::PathBuf;

use polyglot_ast::util::Language;
use polyglot_ast::{PolyglotTree, TreePrinter};

fn main() {
    let file = PathBuf::from("TestSamples/JavaTest.java");
    let tree =
        PolyglotTree::from_path(file, Language::Java).expect("Should not have parsing issues");
    let mut tp = TreePrinter::new();
    tree.apply(&mut tp);
    println!("{}", tp.get_result())
}
