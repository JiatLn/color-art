use anyhow::Result;

use crate::{ conversion, Color, ColorSpace };

impl Color {
    /// Increase the saturation of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to increase the saturation by. Must be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#80e619").unwrap();
    /// color.saturate(0.2).unwrap();
    /// assert_eq!(color.hex(), "#80ff00");
    /// ```
    pub fn saturate(&mut self, amount: f64) -> Result<Self> {
        if amount.abs() > 1.0 {
            anyhow::bail!("Amount must be between 0.0 and 1.0");
        }
        let color = self.vec_of(ColorSpace::HSL);
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let s = (s + amount).min(1.0).max(0.0);
        self.rgb = conversion::hsl::hsl2rgb((h, s, l));
        Ok(*self)
    }
    /// Decrease the saturation of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to decrease the saturation by. Must be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#80e619").unwrap();
    /// color.desaturate(0.2).unwrap();
    /// assert_eq!(color.hex(), "#80cc33");
    /// ```
    pub fn desaturate(&mut self, amount: f64) -> Result<Self> {
        self.saturate(-amount)
    }
    /// greyscale
    ///
    /// Remove all saturation from a color in the HSL color space.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#80e619").unwrap();
    /// color.greyscale().unwrap();
    /// assert_eq!(color.hex(), "#808080");
    /// ```
    pub fn greyscale(&mut self) -> Result<Self> {
        self.desaturate(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn saturate() {
        let mut color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        color.saturate(0.2).unwrap();
        assert_eq!(color.hsl(), "hsl(60, 100%, 50%)");
    }

    #[test]
    fn desaturate() {
        let mut color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        color.desaturate(0.2).unwrap();
        assert_eq!(color.hsl(), "hsl(60, 60%, 50%)");
    }

    #[test]
    fn greyscale() {
        let mut color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        color.greyscale().unwrap();
        assert_eq!(color.hex(), "#808080");

        let mut color = Color::from_str("hsl(90, 0%, 50%)").unwrap();
        color.greyscale().unwrap();
        assert_eq!(color.hex(), "#808080");

        let mut color = Color::from_str("hsl(0, 0%, 50%)").unwrap();
        color.greyscale().unwrap();
        assert_eq!(color.hex(), "#808080");
    }
}
