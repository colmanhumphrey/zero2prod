use unicode_segmentation::UnicodeSegmentation;

pub fn generic_string_checks(s: &str) -> bool {
    let empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_chars = s.chars().filter(|g| forbidden_chars.contains(g)).count() > 0;

    !(empty_or_whitespace || is_too_long || contains_forbidden_chars)
}
