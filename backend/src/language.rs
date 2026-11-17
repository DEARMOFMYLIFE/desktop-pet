use std::env;

/// Resolve the language used for backend-generated text. The UI resolves
/// "system" through the platform locale, so backend notifications do the same.
pub fn is_english(language: &str) -> bool {
    match language {
        "en-US" => true,
        "zh-CN" => false,
        _ => system_locale().starts_with("en"),
    }
}

fn system_locale() -> String {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
        if let Ok(value) = env::var(key) {
            if !value.trim().is_empty() {
                return value.to_ascii_lowercase();
            }
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::is_english;

    #[test]
    fn explicit_languages_are_resolved() {
        assert!(is_english("en-US"));
        assert!(!is_english("zh-CN"));
    }
}
