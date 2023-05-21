use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

/// HSV(Hue, Saturation, Value)
pub fn parse_hsv_str(hsv_str: impl ToString) -> Result<Vec<f64>> {
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

    Ok(hsv_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsv_str() {
        let hsv_str = "hsv(60°,100%,50%)";
        let hsv_vec = parse_hsv_str(hsv_str).unwrap();
        assert_eq!(hsv_vec[0], 60.0);
        assert_eq!(hsv_vec[1], 1.0);
        assert_eq!(hsv_vec[2], 0.5);

        let hsv_str = "hsv(60,1,0.5)";
        let hsv_vec = parse_hsv_str(hsv_str).unwrap();
        assert_eq!(hsv_vec[0], 60.0);
        assert_eq!(hsv_vec[1], 1.0);
        assert_eq!(hsv_vec[2], 0.5);
    }
}
