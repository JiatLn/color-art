use crate::{helper::vec2tuple, ColorSpace};
use anyhow::Result;
use std::str::FromStr;

/// HWB (Hue, Whiteness, Blackness)
pub fn parse_hwb_str(hwb_str: impl ToString) -> Result<(f64, f64, f64)> {
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
                val / 100.
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect::<Vec<f64>>();

    ColorSpace::HWB.valid(&hwb_vec)?;

    Ok(vec2tuple(hwb_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hwb_str() {
        let hwb_str = "hwb(262, 23%, 48%)";
        let (h, w, b) = parse_hwb_str(hwb_str).unwrap();
        assert_eq!(h, 262.);
        assert_eq!(w, 0.23);
        assert_eq!(b, 0.48);

        let hwb_str = "hwb(262, 0.23, 0.48)";
        let (h, w, b) = parse_hwb_str(hwb_str).unwrap();
        assert_eq!(h, 262.);
        assert_eq!(w, 0.23);
        assert_eq!(b, 0.48);
    }
}
