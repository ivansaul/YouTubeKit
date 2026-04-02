use std::str::FromStr;

/// Parse a string after removing all non-numeric characters
pub fn parse_numeric<F>(string: &str) -> Result<F, F::Err>
where
    F: FromStr,
{
    let mut buf = String::new();
    for c in string.chars() {
        if c.is_ascii_digit() {
            buf.push(c);
        }
    }
    buf.parse()
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1.000", 1000)]
    #[case("4 Hello World 2", 42)]
    #[case("50,740 views", 50740)]
    fn test_parse_num(#[case] string: &str, #[case] expect: u32) {
        let n = parse_numeric::<u32>(string).unwrap();
        assert_eq!(n, expect);
    }
}
