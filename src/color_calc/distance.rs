use crate::{Color, ColorSpace};

/// Computes the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance#Three_dimensions) between two colors.
pub fn distance(color1: Color, color2: Color) -> f64 {
    let vec1 = color1.space(ColorSpace::RGBA).unwrap();
    let vec2 = color2.space(ColorSpace::RGBA).unwrap();

    let mut d = 0.0;

    vec1.iter().zip(vec2.iter()).for_each(|(a, b)| {
        d += (a - b).powf(2.0);
    });

    d.sqrt()
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
