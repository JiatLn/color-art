use crate::ColorSpace;
use anyhow::Result;

pub fn parse_rgb_str(s: impl ToString) -> Result<Vec<f64>> {
    let s = s
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("rgb(", "")
        .replace(")", "");

    let mut s = s.split(",");

    let r = s.next().unwrap().parse::<f64>()?;
    let g = s.next().unwrap().parse::<f64>()?;
    let b = s.next().unwrap().parse::<f64>()?;

    let rgb = vec![r, g, b];

    ColorSpace::RGB.valid(&rgb)?;

    Ok(rgb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_rgb() {
        let s = "rgb(255, 255, 255)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![255.0, 255.0, 255.0]);

        let s = "rgb(0, 0, 0)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_parser_rgb_error() {
        let s = "rgb255, 0, 0)";
        let s = parse_rgb_str(s);
        assert!(s.is_err());

        let s = "rgb(2555, 0, 0)";
        let s = parse_rgb_str(s);
        assert!(s.is_err());
    }
}
