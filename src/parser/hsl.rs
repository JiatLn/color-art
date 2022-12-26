use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

/// HSL(Hue, Saturation, Lightness)
pub fn parse_hsl_str(hsl_str: impl ToString) -> Result<(f64, f64, f64)> {
    // hsl_str like "hsl(120°, 1, 0.75)"
    let hsl_str = hsl_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("°", "")
        .replace("hsl(", "")
        .replace(")", "");

    let hsl_str = hsl_str
        .split(",")
        .map(|s| {
            if s.contains('%') {
                f64::from_str(s.replace("%", "").as_str()).unwrap() / 100.
            } else {
                f64::from_str(s).unwrap()
            }
        })
        .collect::<Vec<f64>>();

    let h = hsl_str[0];
    let s = hsl_str[1];
    let l = hsl_str[2];

    ColorSpace::HSL.valid(vec![h, s, l])?;

    Ok((h, s, l))
}
