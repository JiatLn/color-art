use crate::Color;
use anyhow::{bail, Ok, Result};

impl Color {
    /// Mix two colors with a weight.
    ///
    /// # Arguments
    ///
    /// * `color1` - The first color.
    /// * `color2` - The second color.
    /// * `weight` - The weight of the first color. Must be between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let color1 = Color::from_str("#998099").unwrap();
    /// let color2 = Color::from_str("midnightblue").unwrap();
    /// let color3 = Color::mix(color1, color2, 0.5).unwrap();
    /// assert_eq!(color3.hex(), "#594d85");
    /// ```
    pub fn mix(color1: Color, color2: Color, weight: f64) -> Result<Color> {
        if weight < 0.0 || weight > 1.0 {
            bail!("weight must be between 0.0 and 1.0");
        }
        let rgba1 = color1.rgba;
        let rgba2 = color2.rgba;
        let w1 = weight;
        let w2 = 1.0 - weight;
        let r = rgba1.0 * w1 + rgba2.0 * w2;
        let g = rgba1.1 * w1 + rgba2.1 * w2;
        let b = rgba1.2 * w1 + rgba2.2 * w2;
        let a = rgba1.3 * w1 + rgba2.3 * w2;
        Ok(Color::new(r, g, b, a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_mix() {
        let color1 = Color::from_str("#003366").unwrap();
        let color2 = Color::from_str("#d2e1dd").unwrap();

        let color3 = Color::mix(color1, color2, 0.5).unwrap();
        assert_eq!(color3.hex(), "#698aa2");

        let color4 = Color::mix(color1, color2, 0.25).unwrap();
        assert_eq!(color4.hex(), "#9eb6bf");

        let color5 = Color::mix(color1, color2, 0.75).unwrap();
        assert_eq!(color5.hex(), "#355f84");

        let color6 = Color::mix(color1, color2, 0.0).unwrap();
        assert_eq!(color6.hex(), "#d2e1dd");

        let color7 = Color::mix(color1, color2, 1.0).unwrap();
        assert_eq!(color7.hex(), "#003366");
    }
}
