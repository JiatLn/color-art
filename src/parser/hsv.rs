use crate::{ helper::vec2tuple, ColorSpace };
use anyhow::Result;
use std::str::FromStr;

/// HSV(Hue, Saturation, Value)
pub fn parse_hsv_str(hsv_str: impl ToString) -> Result<(f64, f64, f64)> {
    // hsv_str like "hsv(60°,100%,50%)"
    let hsv_str = hsv_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsv(", "")
        .replace(")", "");

    let hsv_vec = hsv_str
        .split(",")
        .map(|s| {
            if s.contains('%') {
                let val = f64::from_str(s.replace("%", "").as_str()).unwrap();
                val / 100.0
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect::<Vec<f64>>();

    ColorSpace::HSV.valid(&hsv_vec)?;

    Ok(vec2tuple(hsv_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsv_str() {
        let hsv_str = "hsv(60°,100%,50%)";
        let (h, s, v) = parse_hsv_str(hsv_str).unwrap();
        assert_eq!(h, 60.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 0.5);

        let hsv_str = "hsv(60,1,0.5)";
        let (h, s, v) = parse_hsv_str(hsv_str).unwrap();
        assert_eq!(h, 60.0);
        assert_eq!(s, 1.0);
        assert_eq!(v, 0.5);
    }
}
