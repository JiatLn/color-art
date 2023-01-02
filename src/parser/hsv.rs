use crate::ColorSpace;
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
                val / 100.
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect::<Vec<f64>>();

    ColorSpace::HSV.valid(&hsv_vec)?;

    let h = hsv_vec[0];
    let s = hsv_vec[1];
    let v = hsv_vec[2];

    Ok((h, s, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsv_str() {
        let hsv_str = "hsv(60°,100%,50%)";
        let (h, s, v) = parse_hsv_str(hsv_str).unwrap();
        assert_eq!(h, 60.);
        assert_eq!(s, 1.);
        assert_eq!(v, 0.5);

        let hsv_str = "hsv(60,1,0.5)";
        let (h, s, v) = parse_hsv_str(hsv_str).unwrap();
        assert_eq!(h, 60.);
        assert_eq!(s, 1.);
        assert_eq!(v, 0.5);
    }
}
