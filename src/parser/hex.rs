use anyhow::{anyhow, Result};

pub fn parse_hex_str(s: &str) -> Result<String> {
    let len = s.len();
    // #rgb #rrggbb #rrggbbaa #rgba
    if !s.starts_with('#')
        || (len != 9 && len != 7 && len != 5 && len != 4)
        || !s.chars().skip(1).all(|c| c.is_ascii_hexdigit())
    {
        return Err(anyhow!("Invalid hex string: {}", s));
    }
    Ok(s.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_hex_str() {
        let s = "#ffffff";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#ffffff");

        let s = "#000";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#000");

        let s = "#0000oo";
        let hex = parse_hex_str(s);
        assert!(hex.is_err());

        let s = "#00000000";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#00000000");

        let s = "#000000000";
        let hex = parse_hex_str(s);
        assert!(hex.is_err());
    }
}
