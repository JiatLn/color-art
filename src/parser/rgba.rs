use crate::ColorSpace;
use anyhow::Result;

pub fn parse_rgba_str(s: impl ToString) -> Result<(f64, f64, f64, f64)> {
    let s = s
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("rgba(", "")
        .replace(")", "");

    let mut s = s.split(",");

    let r = s.next().unwrap().parse::<f64>()?;
    let g = s.next().unwrap().parse::<f64>()?;
    let b = s.next().unwrap().parse::<f64>()?;
    let a = s.next().unwrap().parse::<f64>()?;

    ColorSpace::RGBA.valid(&vec![r, g, b, a])?;

    Ok((r, g, b, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_rgba() {
        let s = "rgba(255, 255, 255, 1)";
        let (r, g, b, a) = parse_rgba_str(s).unwrap();
        assert_eq!((r, g, b, a), (255.0, 255.0, 255.0, 1.0));

        let s = "rgba(0, 0, 0, 0)";
        let (r, g, b, a) = parse_rgba_str(s).unwrap();
        assert_eq!((r, g, b, a), (0.0, 0.0, 0.0, 0.0));

        let s = "rgba(255, 0, 0, 0.5)";
        let (r, g, b, a) = parse_rgba_str(s).unwrap();
        assert_eq!((r, g, b, a), (255.0, 0.0, 0.0, 0.5));

        let s = "rgba(255, 0, 0, 5)";
        let s = parse_rgba_str(s);
        assert!(s.is_err());
    }
}
