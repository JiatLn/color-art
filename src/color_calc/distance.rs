use crate::{Color, ColorSpace};

/// Computes the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance#Three_dimensions) between two colors in a given color space.
///
/// # Examples
///
/// ```
/// use color_art::{distance_with_space, Color, ColorSpace};
/// use std::str::FromStr;
///
/// let color1 = Color::from_str("#fefe0e").unwrap();
/// let color2 = Color::from_str("#fff").unwrap();
///
/// let d = distance_with_space(&color1, &color2, ColorSpace::HSL);
/// assert_eq!(d, 60.01000749874974);
/// ```
pub fn distance_with_space(color1: &Color, color2: &Color, color_space: ColorSpace) -> f64 {
    let vec1 = color1.space(color_space.clone()).unwrap();
    let vec2 = color2.space(color_space.clone()).unwrap();

    let mut d = 0.0;

    vec1.iter().zip(vec2.iter()).for_each(|(a, b)| {
        d += (a - b).powf(2.0);
    });

    d.sqrt()
}

/// Computes the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance#Three_dimensions) between two colors in RGBA color space.
///
/// # Examples
///
/// ```
/// use color_art::{distance, Color};
/// use std::str::FromStr;
///
/// let color1 = Color::from_str("#fefe0e").unwrap();
/// let color2 = Color::from_str("#fff").unwrap();
///
/// let d = distance(&color1, &color2);
/// assert_eq!(d, 241.00414934187336);
/// ```
pub fn distance(color1: &Color, color2: &Color) -> f64 {
    distance_with_space(color1, color2, ColorSpace::RGBA)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_distance() {
        let color1 = Color::from_str("#fefe0e").unwrap();
        let color2 = Color::from_str("#fff").unwrap();

        let d = distance(&color1, &color2);
        assert_eq!(d, 241.00414934187336);

        let d = distance_with_space(&color1, &color2, ColorSpace::RGB);
        assert_eq!(d, 241.00414934187336);

        let d = distance_with_space(&color1, &color2, ColorSpace::HSL);
        assert_eq!(d, 60.01000749874974);
    }
}
