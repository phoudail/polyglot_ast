use super::*;
use std::path::PathBuf;

#[test]
fn python_test() {
    let file = PathBuf::from("TestSamples/export_x.py");
    PolyglotTree::from_path(file, util::Language::Python);
}

#[test]
fn js_test() {
    let file = PathBuf::from("TestSamples/test_pyprint.js");
    PolyglotTree::from_path(file, util::Language::JavaScript);
}

#[test]
fn js_test_file() {
    let file = PathBuf::from("TestSamples/test_pyprint_file.js");
    PolyglotTree::from_path(file, util::Language::JavaScript);
}

#[test]
fn java_test() {
    let file = PathBuf::from("TestSamples/JavaTest.java");
    let expected = PathBuf::from("TestSamples/JavaTest_expected.txt");
    let tree = PolyglotTree::from_path(file, util::Language::Java).expect("AST creation failed for test file TestSamples/JavaTest_expected.txt");
    let expected = std::fs::read_to_string(expected).expect("Missing test file TestSamples/JavaTest_expected.txt");
    let mut tp = TreePrinter::new();
    tree.apply(&mut tp);
    let actual = tp.get_result();
    assert_eq!(expected, actual);
}
