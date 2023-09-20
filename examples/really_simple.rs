//! A simple example building and using the Polyglote AST.

use tree_sitter::{Node, Parser, Tree};

use polyglot_ast::prelude::*;
use polyglot_ast::Language;
use polyglot_ast::PolyBuilder;
use polyglot_ast::PolyStuff;
use polyglot_ast::PolyglotTree;
use polyglot_ast::TreePrinter;

use std::path::PathBuf;

pub(crate) fn polyglot_syntax_tree(
    code: std::sync::Arc<str>,
    lang: polyglot_ast::util::Language,
) -> String {
    let parse = PolyglotTree::parse(code, lang);

    let mut result = parse.tree();
    let zipper = polyglot_ast::PolyglotZipper::new(result.as_ref().unwrap());
    
    let tree_printer = &mut polyglot_ast::TreePrinter::new();
    tree_printer.process_impl(zipper);
    return tree_printer.get_result().to_string()

    //format!("{:#?}", parse.tree())
}

/// A simple processor that pretty prints the polyglot AST.
fn main() {
    let code = std::sync::Arc::from("print('hello')");
    let lang = Language::Python;
    let result = polyglot_syntax_tree(code, lang);
    println!("{}", result);
}
