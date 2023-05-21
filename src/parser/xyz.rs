use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

pub fn parse_xyz_str(xyz_str: impl ToString) -> Result<Vec<f64>> {
    // xyz_str like "xyz(0.412453, 0.212671, 0.019334)"
    let xyz_str = xyz_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("xyz(", "")
        .replace(")", "");

    let xyz_vec = xyz_str
        .split(",")
        .map(|s| f64::from_str(s).unwrap())
        .collect::<Vec<f64>>();

    ColorSpace::XYZ.valid(&xyz_vec)?;

    Ok(xyz_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xyz_str() {
        let xyz_str = "xyz(0.412453, 0.212671, 0.019334)";
        let xyz = parse_xyz_str(xyz_str).unwrap();
        assert_eq!(xyz[0], 0.412453);
        assert_eq!(xyz[1], 0.212671);
        assert_eq!(xyz[2], 0.019334);
    }
}
