use crate::{helper::vec2tuple, ColorSpace};
use anyhow::Result;
use std::str::FromStr;

pub fn parse_yuv_str(yuv_str: impl ToString) -> Result<(f64, f64, f64)> {
    // yuv_str like "yuv(0.299, -0.1474, 0.6148)"
    let yuv_str = yuv_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("yuv(", "")
        .replace(")", "");

    let yuv_vec = yuv_str
        .split(",")
        .map(|s| f64::from_str(s).unwrap())
        .collect::<Vec<f64>>();

    ColorSpace::YUV.valid(&yuv_vec)?;

    Ok(vec2tuple(yuv_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yuv_str() {
        let yuv_str = "yuv(0.299, -0.1474, 0.6148)";
        let (y, u, v) = parse_yuv_str(yuv_str).unwrap();
        assert_eq!(y, 0.299);
        assert_eq!(u, -0.1474);
        assert_eq!(v, 0.6148);
    }
}
