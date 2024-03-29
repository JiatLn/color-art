use crate::{Color, Error};

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
    /// ```rust
    /// use color_art::{Color, color};
    ///
    /// let color1 = color!(#998099);
    /// let color2 = color!(#191970);
    /// let color3 = Color::mix(&color1, &color2, 0.5).unwrap();
    /// assert_eq!(color3.hex(), "#594d85");
    /// ```
    pub fn mix(color1: &Color, color2: &Color, weight: f64) -> Result<Self, Error> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(Error::InvalidParamsError(
                "weight must be between 0.0 and 1.0".to_string(),
            ));
        }
        let (w1, w2) = (weight, 1.0 - weight);
        let r = color1.rgb[0] * w1 + color2.rgb[0] * w2;
        let g = color1.rgb[1] * w1 + color2.rgb[1] * w2;
        let b = color1.rgb[2] * w1 + color2.rgb[2] * w2;
        let a = color1.alpha * w1 + color2.alpha * w2;
        Ok(Color::new(r, g, b, a))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_mix() {
        let color1 = color!(#003366);
        let color2 = color!(#d2e1dd);

        let color3 = Color::mix(&color1, &color2, 0.5).unwrap();
        assert_eq!(color3.hex(), "#698aa2");

        let color4 = Color::mix(&color1, &color2, 0.25).unwrap();
        assert_eq!(color4.hex(), "#9eb6bf");

        let color5 = Color::mix(&color1, &color2, 0.75).unwrap();
        assert_eq!(color5.hex(), "#355f84");

        let color6 = Color::mix(&color1, &color2, 0.0).unwrap();
        assert_eq!(color6.hex(), "#d2e1dd");

        let color7 = Color::mix(&color1, &color2, 1.0).unwrap();
        assert_eq!(color7.hex(), "#036");
    }

    #[test]
    fn test_mix_error() {
        let color1 = color!(#003366);
        let color2 = color!(#d2e1dd);
        let result = Color::mix(&color1, &color2, 1.1);
        assert!(result.is_err());
    }
}
