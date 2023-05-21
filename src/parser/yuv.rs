use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

pub fn parse_yuv_str(yuv_str: impl ToString) -> Result<Vec<f64>> {
    // yuv_str like "yuv(0.299, -0.1471, 0.6148)"
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

    Ok(yuv_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yuv_str() {
        let yuv_str = "yuv(0.299, -0.1471, 0.6148)";
        let yuv = parse_yuv_str(yuv_str).unwrap();
        assert_eq!(yuv[0], 0.299);
        assert_eq!(yuv[1], -0.1471);
        assert_eq!(yuv[2], 0.6148);
    }
}
