use crate::{Color, ColorSpace};

/// Computes the [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance#Three_dimensions) between two colors in a given color space.
///
/// # Examples
///
/// ```
/// use color_art::{distance_with, color, ColorSpace};
///
/// let color1 = color!(#fefe0e);
/// let color2 = color!(#fff);
///
/// let d = distance_with(&color1, &color2, ColorSpace::HSL);
/// assert_eq!(d, 60.01007164576413);
/// ```
pub fn distance_with(color1: &Color, color2: &Color, color_space: ColorSpace) -> f64 {
    let vec1 = color1.vec_of(color_space);
    let vec2 = color2.vec_of(color_space);

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
/// use color_art::{distance, color};
///
/// let color1 = color!(#fefe0e);
/// let color2 = color!(#fff);
///
/// let d = distance(&color1, &color2);
/// assert_eq!(d, 241.00414934187336);
/// ```
pub fn distance(color1: &Color, color2: &Color) -> f64 {
    distance_with(color1, color2, ColorSpace::RGBA)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_distance() {
        let color1 = color!(#fefe0e);
        let color2 = color!(#fff);

        let d = distance(&color1, &color2);
        assert_eq!(d, 241.00414934187336);

        let d = distance_with(&color1, &color2, ColorSpace::RGB);
        assert_eq!(d, 241.00414934187336);

        let d = distance_with(&color1, &color2, ColorSpace::HSL);
        assert_eq!(d, 60.01007164576413);
    }
}
