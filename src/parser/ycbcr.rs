use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

pub fn parse_ycbcr_str(ycbcr_str: impl ToString) -> Result<Vec<f64>> {
    // ycbcr_str like "YCbCr(225.93, 0.5755, 148.7269)"
    let ycbcr_str = ycbcr_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("ycbcr(", "")
        .replace(")", "");

    let ycbcr_vec = ycbcr_str
        .split(",")
        .map(|s| f64::from_str(s).unwrap())
        .collect::<Vec<f64>>();

    ColorSpace::YCbCr.valid(&ycbcr_vec)?;

    Ok(ycbcr_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ycbcr_str() {
        let ycbcr_str = "YCbCr(225.93, 0.5755, 148.7269)";
        let ycbcr = parse_ycbcr_str(ycbcr_str).unwrap();
        assert_eq!(ycbcr[0], 225.93);
        assert_eq!(ycbcr[1], 0.5755);
        assert_eq!(ycbcr[2], 148.7269);
    }
}
