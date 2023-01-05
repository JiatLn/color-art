use crate::Color;

/// Computes the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance#Three_dimensions) between two colors.
pub fn distance(color1: Color, color2: Color) -> f64 {
    let rgba1 = color1.space(crate::ColorSpace::RGBA).unwrap();
    let rgba2 = color2.space(crate::ColorSpace::RGBA).unwrap();

    let mut d = 0.0;

    for i in 0..rgba1.len() {
        d += (rgba1[i] - rgba2[i]).powf(2.0)
    }

    d.powf(0.5)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_distance() {
        let color1 = Color::from_str("#fefe0e").unwrap();
        let color2 = Color::from_str("#fff").unwrap();

        let d = distance(color1, color2);
        assert_eq!(d, 241.00414934187336);
    }
}
