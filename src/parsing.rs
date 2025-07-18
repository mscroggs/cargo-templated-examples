//! String parsing

/// If a string starts with a quote, parse what's inside the quotes
pub fn parse_string_if_quoted(s: &str) -> String {
    if s.starts_with("\"") || s.starts_with("'") {
        assert_eq!(s[0..1], s[s.len() - 1..s.len()]);
        let mut s = s[1..s.len() - 1].chars();
        let mut output = String::new();
        while let Some(c) = s.next() {
            if c == '\\' {
                output.push(s.next().expect("String cannot end with a backslash"));
            } else {
                output.push(c);
            }
        }
        output
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_if_quoted() {
        assert_eq!(parse_string_if_quoted("\"test\""), "test");
        assert_eq!(parse_string_if_quoted("\"test\\\\\""), "test\\");
        assert_eq!(parse_string_if_quoted("\"test\\\"\""), "test\"");
        assert_eq!(parse_string_if_quoted("test\\\""), "test\\\"");
    }
}
