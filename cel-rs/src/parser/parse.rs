use unescape::unescape;

pub fn parse_str(str: &str) -> String {
    unescape(str).unwrap_or(String::new())
}