//! closer to usage with lsp.
//!  
//! TODO

use std::path::PathBuf;

use ::polyglot_ast::building::PolyglotUse;
use ::polyglot_ast::context::GlobalContext;
use polyglot_ast::PolyglotTree;

pub struct FileId(pub u32);

fn polyglot_parse_query(
    //db: &dyn SourceDatabase,
    path: PathBuf,
    lang: polyglot_ast::util::Language,
) -> polyglot_ast::RawParseResult {
    //let _p = profile::span("polyglot_parse_query").detail(|| format!("{file_id:?}"));
    //TODO: edit text value
    // convert file to string
    let text = std::fs::read_to_string(path).unwrap();
    // polyglot_ast::PolyglotTree::parse(text.to_string().into(), lang)
    polyglot_ast::parse(text.into(), lang)
}

pub(crate) fn polyglot_syntax_tree(
    path: PathBuf,
    code: std::sync::Arc<str>,
    lang: polyglot_ast::util::Language,
) -> String {
    let text = std::fs::read_to_string(path).unwrap();
    // polyglot_ast::PolyglotTree::parse(text.to_string().into(), lang)
    let parse = polyglot_ast::parse(text.into(), lang);

    //transform parse into Parsing Result type
    let mut result = parse.cst.unwrap();

    //transform Parsing Result type into Polyglot Tree type
    let result = todo!(); //PolyglotTree::;

    let zipper = polyglot_ast::PolyglotZipper::new(result);

    let tree_printer = &mut polyglot_ast::TreePrinter::new();
    tree_printer.process_impl(zipper);
    return tree_printer.get_result().to_string();
}

fn main() {
    // GlobalContext::;
    // PolyglotUse::

    //declare the file and language
    let file = PathBuf::from("TestSamples/JavaTest3.java");
    let lang = polyglot_ast::util::Language::Java;

    //parsing
    let parse = polyglot_parse_query(file, lang);

    //transform parse into polyglot tree without using method new
    //let mut tree = polyglot_ast::PolyglotTree::

    //print the tree
    //let mut tp = polyglot_ast::TreePrinter::new();
    //tree.apply(&mut tp);

    //return the result
    println!("{:?}", parse);
}
