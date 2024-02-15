use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid argument received")]
pub struct InvalidArgumentError;

/// An enumeration that represents all languages supported by this crate. Current options are Python, JavaScript and Java.
pub enum Language {
    Python,
    JavaScript,
    /// Warning: Java language support is very partial and limited to string literal usage. Keep this in mind when writing your programs
    Java,
}

/// Returns a String identical to the provided slice but with leading and trailing characters removed.
/// In practice, this is mostly used to remove quotes from string literals, but the function does not actually check which characters it removes.
///
/// # Examples
/// ```
/// use polyglot_ast::util;
///
/// let s = "\'Hello!\'";
/// let stripped = util::strip_quotes(&s);
/// assert_eq!(stripped, String::from("Hello!"));
///
/// let stripped_again = util::strip_quotes(stripped.as_str());
/// assert_eq!(stripped_again, String::from("ello"));
///
/// ```
pub fn strip_quotes(s: &str) -> String {
    let mut tmp = s.chars();
    tmp.next();
    tmp.next_back();
    String::from(tmp.as_str())
}

/// Returns the treesitter language corresponding to the string slice passed.
///
/// If the string slice does not match any supported language, the return value will be an InvalidArgumentError.
///
/// # Examples
/// Valid use-case:
/// ```
/// use polyglot_ast::util;
///
/// let language = util::language_string_to_treesitter("python").expect("Python is a supported polyglot AST language");
///
/// assert_eq!(language, tree_sitter_python::language());
/// ```
/// Invalid use-case:
/// ```
/// use polyglot_ast::util;
/// use util::InvalidArgumentError;
///
/// let language = util::language_string_to_treesitter("go");
/// let invalid: InvalidArgumentError = match language {
///     Ok(_) => panic!("Go is not a supported language"),
///     Err(e) => e,
/// };
/// ```
pub fn language_string_to_treesitter(
    lang: &str,
) -> Result<tree_sitter::Language, InvalidArgumentError> {
    Ok(language_enum_to_treesitter(&language_string_to_enum(lang)?))
}

/// Returns the treesitter language corresponding to the Language enum reference passed.
///
/// # Example
/// ```
/// use polyglot_ast::util;
/// use util::Language;
///
/// let language = util::language_enum_to_treesitter(&Language::Python);
///
/// assert_eq!(language, tree_sitter_python::language());
/// ```
pub fn language_enum_to_treesitter(lang: &Language) -> tree_sitter::Language {
    match lang {
        Language::Python => tree_sitter_python::language(),
        Language::JavaScript => tree_sitter_javascript::language(),
        Language::Java => tree_sitter_java::language(),
    }
}

/// Returns the Language enum corresponding to the passed string slice
/// If the string slice does not match any supported language, the return value will be an InvalidArgumentError.
/// # Examples
/// Valid use-case:
/// ```
/// use polyglot_ast::util;
/// use util::Language;
///
/// let language = util::language_string_to_enum("python").expect("Python is a supported polyglot AST language");
///
/// assert!(matches!(language, Language::Python));
/// ```
/// Invalid use-case:
/// ```
/// use polyglot_ast::util;
/// use util::InvalidArgumentError;
///
/// let language = util::language_string_to_treesitter("go");
/// let invalid: InvalidArgumentError = match language {
///     Ok(_) => panic!("Go is not a supported language"),
///     Err(e) => e,
/// };
/// ```
pub fn language_string_to_enum(lang: &str) -> Result<Language, InvalidArgumentError> {
    match lang {
        "python" => Ok(Language::Python),
        "js" | "javascript" => Ok(Language::JavaScript),
        "java" => Ok(Language::Java),
        _ => Err(InvalidArgumentError),
    }
}
