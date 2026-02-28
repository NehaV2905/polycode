/// Language detection utilities
///
/// Automatically detects programming language from file extension
/// and validates if it's supported by Module 1.

use std::path::Path;
use anyhow::{Result, anyhow};

/// Supported programming languages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Python,
    Java,
    Go,
    C,
    Ruby,
    Rust,
}

impl Language {
    /// Convert to string representation expected by Module 1
    pub fn as_str(&self) -> &str {
        match self {
            Language::Python => "python",
            Language::Java => "java",
            Language::Go => "go",
            Language::C => "c",
            Language::Ruby => "ruby",
            Language::Rust => "rust",
        }
    }

    /// Get human-readable name
    pub fn display_name(&self) -> &str {
        match self {
            Language::Python => "Python",
            Language::Java => "Java",
            Language::Go => "Go",
            Language::C => "C",
            Language::Ruby => "Ruby",
            Language::Rust => "Rust",
        }
    }
}

/// Detect programming language from file extension
///
/// # Arguments
/// * `file_path` - Path to the source file
///
/// # Returns
/// * `Ok(Language)` - If language is detected and supported
/// * `Err` - If file has no extension or unsupported language
///
/// # Examples
/// ```
/// use module2_ir_builder::{detect_language, Language};
///
/// let lang = detect_language("main.py").unwrap();
/// assert_eq!(lang, Language::Python);
/// ```
pub fn detect_language(file_path: &str) -> Result<Language> {
    let path = Path::new(file_path);

    // Get file extension
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow!("Cannot determine file type: no file extension"))?;

    // Map extension to language
    match extension.to_lowercase().as_str() {
        "py" => Ok(Language::Python),
        "java" => Ok(Language::Java),
        "go" => Ok(Language::Go),
        "c" | "h" => Ok(Language::C),
        "rb" => Ok(Language::Ruby),
        "rs" => Ok(Language::Rust),
        _ => Err(anyhow!(
            "Unsupported language: '.{}' files are not supported\n\
             \nSupported languages:\n  \
             - Python (.py)\n  \
             - Java (.java)\n  \
             - Go (.go)\n  \
             - C (.c, .h)\n  \
             - Ruby (.rb)\n  \
             - Rust (.rs)",
            extension
        )),
    }
}

/// Check if a language is supported
pub fn is_supported(extension: &str) -> bool {
    matches!(
        extension.to_lowercase().as_str(),
        "py" | "java" | "go" | "c" | "h" | "rb" | "rs"
    )
}

/// Get list of all supported extensions
pub fn supported_extensions() -> Vec<&'static str> {
    vec!["py", "java", "go", "c", "h", "rb", "rs"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_python() {
        assert_eq!(detect_language("main.py").unwrap(), Language::Python);
        assert_eq!(detect_language("/path/to/script.py").unwrap(), Language::Python);
    }

    #[test]
    fn test_detect_java() {
        assert_eq!(detect_language("Main.java").unwrap(), Language::Java);
    }

    #[test]
    fn test_detect_go() {
        assert_eq!(detect_language("main.go").unwrap(), Language::Go);
    }

    #[test]
    fn test_detect_c() {
        assert_eq!(detect_language("main.c").unwrap(), Language::C);
        assert_eq!(detect_language("header.h").unwrap(), Language::C);
    }

    #[test]
    fn test_detect_ruby() {
        assert_eq!(detect_language("script.rb").unwrap(), Language::Ruby);
    }

    #[test]
    fn test_detect_rust() {
        assert_eq!(detect_language("main.rs").unwrap(), Language::Rust);
    }

    #[test]
    fn test_unsupported_language() {
        let result = detect_language("file.js");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported language"));
    }

    #[test]
    fn test_no_extension() {
        let result = detect_language("Makefile");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no file extension"));
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(detect_language("Main.PY").unwrap(), Language::Python);
        assert_eq!(detect_language("Main.JAVA").unwrap(), Language::Java);
    }
}
