use anyhow::Result;

use crate::{Color, ColorSpace};

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
    /// let color = Color::from_str("#80e619").unwrap();
    /// let color = color.saturate(0.2).unwrap();
    /// assert_eq!(color.hex(), "#80ff00");
    /// ```
    pub fn saturate(&self, amount: f64) -> Result<Color> {
        if amount.abs() > 1.0 {
            anyhow::bail!("Amount must be between 0.0 and 1.0")
        }
        let color = self.space(ColorSpace::Hsl)?;
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let s = (s + amount).min(1.0).max(0.0);
        Color::from_hsl(h, s, l)
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
    /// let color = Color::from_str("#80e619").unwrap();
    /// let color = color.desaturate(0.2).unwrap();
    /// assert_eq!(color.hex(), "#80cc33");
    /// ```
    pub fn desaturate(&self, amount: f64) -> Result<Color> {
        Ok(self.saturate(-amount)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn saturate() {
        let color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        let color = color.saturate(0.2).unwrap();
        assert_eq!(color.hsl(), "hsl(60, 100%, 50%)");
    }

    #[test]
    fn desaturate() {
        let color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        let color = color.desaturate(0.2).unwrap();
        assert_eq!(color.hsl(), "hsl(60, 60%, 50%)");
    }
}
