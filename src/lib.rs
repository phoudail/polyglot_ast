pub mod util {
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("Invalid argument received")]
    pub struct InvalidArgumentError;

    pub enum Language {
        Python,
        JavaScript,
        Java,
    }

    pub fn language_string_to_treesitter(lang: &str) -> Result<tree_sitter::Language, InvalidArgumentError> {
        Ok(language_enum_to_treesitter(&language_string_to_enum(lang)?))
    }

    pub fn language_enum_to_treesitter(lang: &Language) -> tree_sitter::Language {
        match lang {
            Language::Python => tree_sitter_python::language(),
            Language::JavaScript => tree_sitter_javascript::language(),
            Language::Java => tree_sitter_java::language(),
        }
    }

    pub fn language_string_to_enum(lang: &str) -> Result<Language, InvalidArgumentError> {
        match lang {
            "python" => Ok(Language::Python),
            "js" | "javascript" => Ok(Language::JavaScript),
            "java" => Ok(Language::Java),
            _ => Err(InvalidArgumentError),
        }
    }
}

pub mod polyglot_tree;

#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
