//! A simple example building and using the Polyglote AST.

use tree_sitter::{Node, Parser, Tree};

use polyglot_ast::prelude::*;
use polyglot_ast::Language;
use polyglot_ast::PolyBuilder;
use polyglot_ast::PolyStuff;
use polyglot_ast::PolyglotTree;
use polyglot_ast::TreePrinter;

use std::path::PathBuf;

/// A simple processor that pretty prints the polyglot AST.
fn main() {
    //GlobalContext;

    // for each file
    // parse

    
    // query stuff 
    todo!("a simple example building and using the Polyglot AST");
   
    //TODO: use a real file
    let code = std::sync::Arc::from("print('hello')");
    let language = Language::Python;
       
    let parse = PolyglotTree::parse(code, language);
    
    //let mut tree = PolyglotTree::new(parse);

    //let mut tp = TreePrinter::new();
    //tree.apply(&mut tp);
    println!("{:?}", parse);
    
}