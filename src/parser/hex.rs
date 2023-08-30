use crate::Error;

pub fn parse_hex_str(s: impl ToString) -> Result<String, crate::Error> {
    let s = s.to_string();
    let len = s.len();
    // #rgb #rgba #rrggbb #rrggbbaa
    if !s.starts_with('#')
        || (len != 9 && len != 7 && len != 5 && len != 4)
        || !s.chars().skip(1).all(|c| c.is_ascii_hexdigit())
    {
        Err(Error::ColorParserError(format!(
            "Invalid hex string of '{}'",
            s
        )))
    } else {
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_hex_should_work() {
        let s = "#ffffff";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#ffffff");

        let s = "#000";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#000");

        let s = "#f0ff";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#f0ff");

        let s = "#00000000";
        let hex = parse_hex_str(s).unwrap();
        assert_eq!(hex, "#00000000");
    }

    #[test]
    fn parser_hex_with_error() {
        let s = "#0000oo";
        let hex = parse_hex_str(s);
        assert!(hex.is_err());

        let s = "#000000000";
        let hex = parse_hex_str(s);

        if let Err(err) = hex {
            println!("{}", &err);
            assert_eq!(
                err,
                Error::ColorParserError("Invalid hex string of '#000000000'".to_string())
            );
        } else {
            panic!("should error");
        }
    }
}
