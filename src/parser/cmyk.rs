use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

/// CMYK string parser
///
/// Parse a string as a color in the cmyk format.
/// String like "cmyk(100%, 0, 50%, 50%)"
pub fn parse_cmyk_str(cmyk_str: impl ToString) -> Result<(f64, f64, f64, f64)> {
    let cmyk_str = cmyk_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("cmyk(", "")
        .replace(")", "");

    let cmyk_vec = cmyk_str
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

    ColorSpace::CMYK.valid(&cmyk_vec)?;

    let c = cmyk_vec[0];
    let m = cmyk_vec[1];
    let y = cmyk_vec[2];
    let k = cmyk_vec[3];

    Ok((c, m, y, k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cmyk_str() {
        let cmyk = parse_cmyk_str("cmyk(0.1, 0.2, 0.3, 0.4)").unwrap();
        assert_eq!(cmyk, (0.1, 0.2, 0.3, 0.4));

        let cmyk = parse_cmyk_str("cmyk(100%, 0, 50%, 50%)").unwrap();
        assert_eq!(cmyk, (1.0, 0.0, 0.5, 0.5));
    }
}
