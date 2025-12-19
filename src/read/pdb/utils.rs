/// Extracts u64 from a string.
pub(crate) fn fast_parse_u64_from_string(s: &str) -> Result<u64, String> {
    let bytes = s.as_bytes();
    let mut result: u64 = 0;
    let mut i = 0;
    let len = bytes.len();

    // Skip leading whitespace
    while i < len && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    // Parse digits
    let start = i;
    while i < len && bytes[i].is_ascii_digit() {
        result = result * 10 + u64::from(bytes[i] - b'0');
        i += 1;
    }

    if i == start {
        return Err("No digits found".to_string());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_parse() {
        assert_eq!(fast_parse_u64_from_string("  100"), Ok(100));
        assert_eq!(fast_parse_u64_from_string("    32"), Ok(32));
        assert_eq!(fast_parse_u64_from_string("9999"), Ok(9999));
        assert_eq!(fast_parse_u64_from_string("     1234"), Ok(1234));
        assert!(fast_parse_u64_from_string(" abc ").is_err());
        assert!(fast_parse_u64_from_string(" ").is_err());
        assert!(fast_parse_u64_from_string("     ").is_err());
        assert!(fast_parse_u64_from_string("").is_err());
    }
}
