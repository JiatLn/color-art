use crate::{ conversion, Color, ColorSpace };
use anyhow::Result;

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
    /// let mut color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
    /// color.spin(30.0).unwrap();
    /// assert_eq!(color.hsl(), "hsl(40, 90%, 50%)");
    ///
    /// let mut color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
    /// color.spin(-30.0).unwrap();
    /// assert_eq!(color.hsl(), "hsl(340, 90%, 50%)");
    /// ```
    pub fn spin(&mut self, angle: f64) -> Result<Self> {
        let color = self.vec_of(ColorSpace::HSL);
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let h = (h + angle) % 360.0;
        let h = if h < 0.0 { h + 360.0 } else { h };
        self.rgb = conversion::hsl::hsl2rgb((h, s, l));
        Ok(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_spin() {
        let mut color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
        color.spin(30.0).unwrap();
        assert_eq!(color.hsl(), "hsl(40, 90%, 50%)");

        let mut color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
        color.spin(-30.0).unwrap();
        assert_eq!(color.hsl(), "hsl(340, 90%, 50%)");
    }
}
