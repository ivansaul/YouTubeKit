use std::{str::FromStr, sync::LazyLock};

use regex::Regex;

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

/// Parse textual video length (e.g. `0:49`, `2:02` or `1:48:18`)
/// and return the duration in seconds.
pub fn parse_video_length(text: &str) -> Option<u32> {
    static VIDEO_LENGTH_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?:(\d+)[:.])?(\d{1,2})[:.](\d{2})").unwrap());
    VIDEO_LENGTH_REGEX.captures(text).map(|cap| {
        let hrs = cap
            .get(1)
            .and_then(|x| x.as_str().parse::<u32>().ok())
            .unwrap_or_default();
        let min = cap
            .get(2)
            .and_then(|x| x.as_str().parse::<u32>().ok())
            .unwrap_or_default();
        let sec = cap
            .get(3)
            .and_then(|x| x.as_str().parse::<u32>().ok())
            .unwrap_or_default();

        hrs * 3600 + min * 60 + sec
    })
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

    #[rstest]
    #[case("0:49", Some(49))]
    #[case("bla 2:02 h3llo w0rld", Some(122))]
    #[case("18:22", Some(1102))]
    #[case("1:48:18", Some(6498))]
    #[case("102:12:39", Some(367_959))]
    #[case("42", None)]
    fn test_parse_video_length(#[case] text: &str, #[case] expect: Option<u32>) {
        let n = parse_video_length(text);
        assert_eq!(n, expect);
    }
}
