use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

pub fn parse_rgb_str(s: impl ToString) -> Result<Vec<f64>> {
    let s = s.to_string().trim().to_lowercase().replace("rgb(", "").replace(")", "");

    let rgb = s
        .split(&[',', ' ', '/'][..])
        .filter(|&s| !s.is_empty())
        .collect::<Vec<_>>();

    if !check_percent(&rgb[..3]) {
        return Err(anyhow::anyhow!("Invalid RGB string"));
    }

    let rgb = rgb
        .iter()
        .enumerate()
        .map(|(idx, &s)| {
            if s.contains('%') {
                match f64::from_str(&s.replace("%", "")) {
                    Ok(value) =>
                        Ok({
                            if idx == 3 { value / 100.0 } else { (value * 255.0) / 100.0 }
                        }),
                    Err(e) => Err(e),
                }
            } else {
                f64::from_str(s)
            }
        })
        .collect::<Vec<_>>();

    if rgb.iter().any(|s| s.is_err()) {
        return Err(anyhow::anyhow!("Invalid RGB string"));
    }

    let rgb = rgb
        .into_iter()
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    match rgb.len() {
        3 => ColorSpace::RGB.valid(&rgb)?,
        4 => ColorSpace::RGBA.valid(&rgb)?,
        _ => {
            return Err(anyhow::anyhow!("Invalid RGB string"));
        }
    }
    Ok(rgb)
}

/// Check if the string contains a percentage value
/// and if all values are percentage values
fn check_percent(s: &[&str]) -> bool {
    s.iter().all(|&s| s.contains("%")) || s.iter().all(|&s| !s.contains("%"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_rgb() {
        let s = "rgb(255, 255, 0)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![255.0, 255.0, 0.0]);

        let s = "rgb(255 255 0)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![255.0, 255.0, 0.0]);

        let s = "rgb(255 0 0 / 0.5)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![255.0, 0.0, 0.0, 0.5]);

        let s = "rgb(50% 50% 50%)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![127.5, 127.5, 127.5]);

        let s = "rgb(255 122 127 / 80%)";
        let rgb = parse_rgb_str(s).unwrap();
        assert_eq!(rgb, vec![255.0, 122.0, 127.0, 0.8]);
    }

    #[test]
    fn test_parser_rgb_error() {
        let s = "rgb255, 0, 0)";
        let s = parse_rgb_str(s);
        assert!(s.is_err());

        let s = "rgb(2555, 0, 0)";
        let s = parse_rgb_str(s);
        assert!(s.is_err());

        let s = "rgb(50% 50% 50)";
        let s = parse_rgb_str(s);
        assert!(s.is_err());

        let s = "rgb(120%, 50%, 50%)";
        let s = parse_rgb_str(s);
        assert!(s.is_err());
    }
}
