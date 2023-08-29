use crate::{Color, ColorSpace};

impl Color {
    /// Rotate the hue angle of a color in either direction.
    ///
    /// # Arguments
    ///
    /// * `angle` - The angle to rotate the hue by. Positive values rotate clockwise, negative values rotate counter-clockwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
    /// let color = color.spin(30.0);
    /// assert_eq!(color.hsl(), "hsl(40, 90%, 50%)");
    ///
    /// let color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
    /// let color = color.spin(-30.0);
    /// assert_eq!(color.hsl(), "hsl(340, 90%, 50%)");
    /// ```
    pub fn spin(&self, angle: f64) -> Self {
        let color = self.vec_of(ColorSpace::HSL);
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let h = (360.0 + h + angle) % 360.0;
        Color::from_hsl(h, s, l).unwrap()
    }
    /// Returns the [complement](https://en.wikipedia.org/wiki/Complementary_colors) of color.
    pub fn complement(&self) -> Self {
        self.spin(180.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn test_color_spin() {
        let color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
        let color = color.spin(30.0);
        assert_eq!(color.hsl(), "hsl(40, 90%, 50%)");

        let color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
        let color = color.spin(-30.0);
        assert_eq!(color.hsl(), "hsl(340, 90%, 50%)");
    }

    #[test]
    fn test_color_complement() {
        let color = color!(#6b717f);
        let color = color.complement();
        assert_eq!(color.hex(), "#7f796b");

        let color = color!(#d2e1dd);
        let color = color.complement();
        assert_eq!(color.hex(), "#e1d2d6");

        let color = color!(#036);
        let color = color.complement();
        assert_eq!(color.hex(), "#630");
    }
}
