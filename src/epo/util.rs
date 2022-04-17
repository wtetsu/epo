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
        assert_eq!(false, is_numeric(""));
        assert_eq!(true, is_numeric("0"));
        assert_eq!(true, is_numeric("1"));
        assert_eq!(true, is_numeric("9"));
        assert_eq!(false, is_numeric("A"));
        assert_eq!(true, is_numeric("123"));
        assert_eq!(true, is_numeric("12345678901234567890"));
        assert_eq!(false, is_numeric("-123"));
        assert_eq!(false, is_numeric("+123"));
        assert_eq!(false, is_numeric("123a"));
        assert_eq!(false, is_numeric("a123"));
        assert_eq!(false, is_numeric("a123b"));
    }
}
