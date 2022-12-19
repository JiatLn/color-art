use anyhow::Result;
use std::str::FromStr;

/// HSL(Hue, Saturation, Lightness)
pub fn parse_hsl_str(hsl_str: &str) -> Result<(f64, f64, f64)> {
    // hsl_str like "hsl(120°, 1, 0.75)"
    let hsl_str = hsl_str
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

    if h < 0. || h >= 360. {
        anyhow::bail!("h must be between 0 and 360, got {}", h);
    }
    if s < 0. || s > 1. {
        anyhow::bail!("s must be between 0 and 1, got {}", s);
    }
    if l < 0. || l > 1. {
        anyhow::bail!("l must be between 0 and 1, got {}", l);
    }

    Ok((h, s, l))
}
