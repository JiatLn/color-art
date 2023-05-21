use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

/// HSLA(Hue, Saturation, Lightness, Alpha)
pub fn parse_hsla_str(hsla_str: impl ToString) -> Result<Vec<f64>> {
    // hsla_str like "hsla(120°, 1, 0.75, 0.6)"
    let hsla_str = hsla_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsla(", "")
        .replace(")", "");

    let hsla_vec = hsla_str
        .split(",")
        .map(|s| {
            if s.contains('%') {
                f64::from_str(s.replace("%", "").as_str()).unwrap() / 100.0
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect::<Vec<_>>();

    ColorSpace::HSLA.valid(&hsla_vec)?;

    Ok(hsla_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsla_str() {
        let s = "hsla(120°, 1, 0.75, 0.6)";
        let hsla = parse_hsla_str(s).unwrap();
        assert_eq!(hsla[0], 120.0);
        assert_eq!(hsla[1], 1.0);
        assert_eq!(hsla[2], 0.75);
        assert_eq!(hsla[3], 0.6);
    }
}
