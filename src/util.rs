use std::{fs::File, io::Read};

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid argument received")]
pub struct InvalidArgumentError;

/// An enumeration that represents all languages supported by this crate. Current options are Python, JavaScript and Java.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

// Function to get the extension of a file
fn get_file_extension(file_path: &str) -> Option<String> {
    // Use rfind to find the last occurrence of '.' in the file path
    match file_path.rfind('.') {
        // If '.' is found, extract the substring after '.' and convert it to lowercase
        Some(idx) => Some(file_path[idx + 1..].to_lowercase()),
        // If '.' is not found, return None
        None => None,
    }
}

// Main function to get the language based on the file extension
fn file_extension_to_enum(file_path: &str) -> Option<Language> {
    // Try to open the file
    if let Ok(mut file) = File::open(file_path) {
        let mut buffer = [0; 1024]; // Buffer size for reading
        // Read the content of the file into the buffer
        if let Ok(read_bytes) = file.read(&mut buffer) {
            // Convert the read bytes into a string
            let content = String::from_utf8_lossy(&buffer[..read_bytes]).to_string();

            // Get the extension of the file
            if let Some(extension) = get_file_extension(file_path) {
                // Determine the language based on the file extension
                match extension.as_str() {
                    "java" => Some(Language::Java),
                    "js" => Some(Language::JavaScript),
                    "py" => Some(Language::Python),
                    // Add other cases for additional languages
                    _ => None,
                }
            } else {
                None // If the extension is not found or not supported, return None
            }
        } else {
            None // In case of a read error, return None
        }
    } else {
        None // In case of an error opening the file, return None
    }
}

