pub fn is_numeric(str: &str) -> bool {
    if str.is_empty() {
        return false;
    }
    str.chars().all(|c| c.is_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_numeric() {
        assert!(!is_numeric(""));
        assert!(is_numeric("0"));
        assert!(is_numeric("1"));
        assert!(is_numeric("9"));
        assert!(!is_numeric("A"));
        assert!(is_numeric("123"));
        assert!(is_numeric("12345678901234567890"));
        assert!(!is_numeric("-123"));
        assert!(!is_numeric("+123"));
        assert!(!is_numeric("123a"));
        assert!(!is_numeric("a123"));
        assert!(!is_numeric("a123b"));
    }
}
