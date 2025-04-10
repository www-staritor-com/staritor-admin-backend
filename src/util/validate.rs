use regex::Regex;
use std::sync::LazyLock;
use validator::ValidationError;

pub static SPECIAL_CHAR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^[a-zA-Z0-9_]+$").unwrap()
});

#[allow(unused)]
pub fn validate_exclude_special_charest(s: &str) -> Result<(), ValidationError> {
    if SPECIAL_CHAR_REGEX.is_match(s) {
        return Ok(());
    }

    Err(ValidationError::new("invalid_characters"))
}
