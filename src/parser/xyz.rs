use crate::helper::vec2tuple;
use anyhow::Result;
use std::str::FromStr;

#[allow(dead_code)]
pub fn parse_xyz_str(xyz_str: impl ToString) -> Result<(f64, f64, f64)> {
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

    // TODO: validate xyz value

    Ok(vec2tuple(xyz_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xyz_str() {
        let xyz_str = "xyz(0.412453, 0.212671, 0.019334)";
        let (x, y, z) = parse_xyz_str(xyz_str).unwrap();
        assert_eq!(x, 0.412453);
        assert_eq!(y, 0.212671);
        assert_eq!(z, 0.019334);
    }
}
