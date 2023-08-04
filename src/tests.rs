use super::*;
use std::path::PathBuf;

#[test]
fn python_test() {
    let file = PathBuf::from("TestSamples/export_x.py");
    PolyglotTree::from_path(file, crate::Language::Python);
}

#[test]
fn js_test() {
    let file = PathBuf::from("TestSamples/test_pyprint.js");
    PolyglotTree::from_path(file, crate::Language::Python);
}

#[test]
fn js_test_file() {
    let file = PathBuf::from("TestSamples/test_pyprint_file.js");
    PolyglotTree::from_path(file, crate::Language::Python);
}

#[test]
fn java_test() {
    let file = PathBuf::from("TestSamples/JavaTest.java");
    PolyglotTree::from_path(file, crate::Language::Python);
}
