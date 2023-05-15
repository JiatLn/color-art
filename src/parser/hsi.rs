use crate::{helper::vec2tuple, ColorSpace};
use anyhow::Result;
use std::str::FromStr;

/// Parse a string in HSI color space to an HSI tuple
pub fn parse_hsi_str(hsi_str: impl ToString) -> Result<(f64, f64, f64)> {
    // hsi_str like "hsi(240°, 100%, 33.33%)"
    let hsi_str = hsi_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsi(", "")
        .replace(")", "");

    let hsi_vec = hsi_str
        .split(",")
        .map(|s| {
            if s.contains('%') {
                f64::from_str(s.replace("%", "").as_str()).unwrap() / 100.
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect::<Vec<f64>>();

    ColorSpace::HSI.valid(&hsi_vec)?;

    Ok(vec2tuple(hsi_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hsi_str() {
        let s = "hsi(240°, 100%, 33.33%)";
        let (h, s, i) = parse_hsi_str(s).unwrap();
        assert_eq!(h, 240.0);
        assert_eq!(s, 1.0);
        assert_eq!(i, 0.3333);
    }
}
