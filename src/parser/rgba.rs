use anyhow::Result;

pub fn parse_rgba_str(s: &str) -> Result<(f64, f64, f64, f64)> {
    let s = s
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("rgba(", "")
        .replace(")", "");

    let mut s = s.split(",");

    let r = s.next().unwrap().parse::<f64>()?;
    let g = s.next().unwrap().parse::<f64>()?;
    let b = s.next().unwrap().parse::<f64>()?;
    let a = s.next().unwrap().parse::<f64>()?;

    if r < 0.0 || r > 255.0 {
        anyhow::bail!("r must be between 0 and 255, got {}", r);
    }
    if g < 0.0 || g > 255.0 {
        anyhow::bail!("g must be between 0 and 255, got {}", g);
    }
    if b < 0.0 || b > 255.0 {
        anyhow::bail!("b must be between 0 and 255, got {}", b);
    }
    if a < 0.0 || a > 1.0 {
        anyhow::bail!("alpha must be between 0 and 1, got {}", a);
    }

    Ok((r, g, b, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_rgba() {
        let s = "rgba(255, 255, 255, 1)";
        let (r, g, b, a) = parse_rgba_str(s).unwrap();
        assert_eq!((r, g, b, a), (255.0, 255.0, 255.0, 1.0));

        let s = "rgba(0, 0, 0, 0)";
        let (r, g, b, a) = parse_rgba_str(s).unwrap();
        assert_eq!((r, g, b, a), (0.0, 0.0, 0.0, 0.0));

        let s = "rgba(255, 0, 0, 0.5)";
        let (r, g, b, a) = parse_rgba_str(s).unwrap();
        assert_eq!((r, g, b, a), (255.0, 0.0, 0.0, 0.5));

        let s = "rgba(255, 0, 0, 5)";
        let s = parse_rgba_str(s);
        assert!(s.is_err());
    }
}
