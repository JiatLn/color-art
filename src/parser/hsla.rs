use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

/// HSLA(Hue, Saturation, Lightness, Alpha)
pub fn parse_hsla_str(hsla_str: impl ToString) -> Result<(f64, f64, f64, f64)> {
    // hsla_str like "hsla(120°, 1, 0.75, 0.6)"
    let hsla_str = hsla_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsla(", "")
        .replace(")", "");

    let mut hsla_vec = hsla_str.split(",").map(|s| {
        if s.contains('%') {
            f64::from_str(s.replace("%", "").as_str()).unwrap() / 100.0
        } else {
            f64::from_str(s).unwrap()
        }
    });

    let h = hsla_vec.next().unwrap();
    let s = hsla_vec.next().unwrap();
    let l = hsla_vec.next().unwrap();
    let a = hsla_vec.next().unwrap();

    ColorSpace::HSLA.valid(&vec![h, s, l, a])?;

    Ok((h, s, l, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsla_str() {
        let s = "hsla(120°, 1, 0.75, 0.6)";
        let (h, s, l, a) = parse_hsla_str(s).unwrap();
        assert_eq!(h, 120.0);
        assert_eq!(s, 1.0);
        assert_eq!(l, 0.75);
        assert_eq!(a, 0.6);
    }
}
