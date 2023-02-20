mod util {
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("Invalid argument received")]
    pub struct InvalidArgumentError;

    pub fn language_string_to_treesitter(lang: &str) -> Result<tree_sitter::Language, InvalidArgumentError> {
        match lang {
            "js" | "javascript" => Ok(tree_sitter_javascript::language()),
            "python" => Ok(tree_sitter_python::language()),
            "java" => Ok(tree_sitter_java::language()),
            _ => Err(InvalidArgumentError),
        }
    }
}

mod polyglot_tree;
mod polyglot_zipper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
