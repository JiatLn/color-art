use anyhow::Result;

pub fn parse_rgba_str(s: impl ToString) -> Result<Vec<f64>> {
    let s = s.to_string().replace("rgba(", "rgb(");
    super::rgb::parse_rgb_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_rgba() {
        let s = "rgba(255, 255, 255, 1)";
        let rgba = parse_rgba_str(s).unwrap();
        assert_eq!(rgba, vec![255.0, 255.0, 255.0, 1.0]);

        let s = "rgba(0, 0, 0, 0)";
        let rgba = parse_rgba_str(s).unwrap();
        assert_eq!(rgba, vec![0.0, 0.0, 0.0, 0.0]);

        let s = "rgba(255, 0, 0, 0.5)";
        let rgba = parse_rgba_str(s).unwrap();
        assert_eq!(rgba, vec![255.0, 0.0, 0.0, 0.5]);

        let s = "rgba(255 255 0 0.5)";
        let rgba = parse_rgba_str(s).unwrap();
        assert_eq!(rgba, vec![255.0, 255.0, 0.0, 0.5]);

        let s = "rgba(255 255 0 / 0.5)";
        let rgba = parse_rgba_str(s).unwrap();
        assert_eq!(rgba, vec![255.0, 255.0, 0.0, 0.5]);
    }

    #[test]
    fn test_parser_rgba_error() {
        let s = "rgba(255, 0, 0, 5)";
        let s = parse_rgba_str(s);
        assert!(s.is_err());

        let s = "rgba(255, 0, 0, 0.5, 0.5)";
        let s = parse_rgba_str(s);
        assert!(s.is_err());
    }
}
