use crate::{Color, ColorSpace};
use anyhow::Result;

impl Color {
    /// Decrease the lightness of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to decrease the lightness by. Must be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let color = Color::from_str("#426105").unwrap();
    /// let color = color.darken(0.1).unwrap();
    /// assert_eq!(color.hex(), "#213003");
    /// ```
    pub fn darken(&self, amount: f64) -> Result<Color> {
        if amount.abs() > 1.0 {
            anyhow::bail!("Amount must be between 0.0 and 1.0")
        }
        let color = self.space(ColorSpace::HSL)?;
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let l = (l - amount).min(1.0).max(0.0);
        Color::from_hsl(h, s, l)
    }
    /// Increase the lightness of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to increase the lightness by. Must be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let color = Color::from_str("#80e619").unwrap();
    /// let color = color.lighten(0.2).unwrap();
    /// assert_eq!(color.hex(), "#b3f075");
    /// ```
    pub fn lighten(&self, amount: f64) -> Result<Color> {
        self.darken(-amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_darken() {
        let color = Color::from_str("#426105").unwrap();
        let color = color.darken(0.1).unwrap();
        assert_eq!(color.hex(), "#213003");

        let color = Color::from_str("#426105").unwrap();
        let color = color.darken(0.5).unwrap();
        assert_eq!(color.hex(), "#000000");

        let color = Color::from_str("#80e619").unwrap();
        let color = color.darken(0.2).unwrap();
        assert_eq!(color.hex(), "#4d8a0f");
    }
}
