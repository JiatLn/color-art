use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

/// HWB (Hue, Whiteness, Blackness)
pub fn parse_hwb_str(hwb_str: impl ToString) -> Result<Vec<f64>> {
    // hwb_str like "hwb(262, 23%, 48%)"
    let hwb_str = hwb_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("hwb(", "")
        .replace(")", "");

    let hwb_vec = hwb_str
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

    ColorSpace::HWB.valid(&hwb_vec)?;

    Ok(hwb_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hwb_str() {
        let hwb_str = "hwb(262, 23%, 48%)";
        let hwb_vec = parse_hwb_str(hwb_str).unwrap();
        assert_eq!(hwb_vec[0], 262.0);
        assert_eq!(hwb_vec[1], 0.23);
        assert_eq!(hwb_vec[2], 0.48);

        let hwb_str = "hwb(262, 0.23, 0.48)";
        let hwb_vec = parse_hwb_str(hwb_str).unwrap();
        assert_eq!(hwb_vec[0], 262.0);
        assert_eq!(hwb_vec[1], 0.23);
        assert_eq!(hwb_vec[2], 0.48);
    }
}
