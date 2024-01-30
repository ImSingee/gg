use super::Config;

pub fn parse(s: &str) -> Result<Config, serde_json::Error> {
    serde_json::from_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let s = r#"{}"#;
        let config = parse(s).unwrap();

        assert_eq!(config, Config::default());
    }
}