use crate::ColorSpace;
use anyhow::Result;
use std::str::FromStr;

pub fn parse_lab_str(lab_str: impl ToString) -> Result<Vec<f64>> {
    // lab_str like "lab(97.14, -21.55, 94.48)"
    let lab_str = lab_str
        .to_string()
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("lab(", "")
        .replace(")", "");

    let lab_vec = lab_str
        .split(",")
        .map(|s| f64::from_str(s).unwrap())
        .collect::<Vec<f64>>();

    ColorSpace::Lab.valid(&lab_vec)?;

    Ok(lab_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_str() {
        let lab_str = "lab(97.14, -21.55, 94.48)";
        let lab = parse_lab_str(lab_str).unwrap();
        assert_eq!(lab[0], 97.14);
        assert_eq!(lab[1], -21.55);
        assert_eq!(lab[2], 94.48);
    }
}
