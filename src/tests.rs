use super::*;
use std::path::PathBuf;

fn assert_ast_eq(test_file: &str, expected_file: &str, lang: util::Language) {
    let file = PathBuf::from(test_file);
    let expected = PathBuf::from(expected_file);

    let tree = PolyglotTree::from_path(file, lang)
        .expect(format!("AST creation failed for test file {test_file}").as_str());
    let expected = std::fs::read_to_string(expected)
        .expect(format!("Missing test file {expected_file}").as_str());

    let mut tp = TreePrinter::new();
    tree.apply(&mut tp);
    let actual = tp.get_result();

    assert_eq!(expected, actual);
}


#[test]
fn python_test() {
    let file_test = "TestSamples/export_x.py";
    let file_expect = "TestSamples/export_x_expected.txt";

    assert_ast_eq(file_test, file_expect, util::Language::Python)

}

#[test]
fn js_test() {
    let file_test = "TestSamples/test_pyprint.js";
    let file_expect = "TestSamples/test_pyprint_expected.txt";

    assert_ast_eq(file_test, file_expect, util::Language::JavaScript)
}

#[test]
fn js_test_file() {
    let file_test = "TestSamples/test_pyprint_file.js";
    let file_expect = "TestSamples/test_pyprint_expected.txt";

    assert_ast_eq(file_test, file_expect, util::Language::JavaScript)

}

#[test]
fn java_test() {
    let file_test = "TestSamples/JavaTest.java";
    let file_expect = "TestSamples/JavaTest_expected.txt";

    assert_ast_eq(file_test, file_expect, util::Language::Java)

}
