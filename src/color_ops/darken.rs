use crate::{ conversion, Color, ColorSpace };
use anyhow::{ Ok, Result };

impl Color {
    /// Decrease the lightness of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to decrease the lightness by. Must be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use color_art::color;
    ///
    /// let mut color = color!(#426105);
    /// color.darken(0.1).unwrap();
    /// assert_eq!(color.hex(), "#213102");
    /// ```
    pub fn darken(&mut self, amount: f64) -> Result<Self> {
        if amount.abs() > 1.0 {
            anyhow::bail!("Amount must be between 0.0 and 1.0");
        }
        let color = self.vec_of(ColorSpace::HSL);
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let l = (l - amount).min(1.0).max(0.0);
        self.rgb = conversion::hsl::hsl2rgb((h, s, l));
        Ok(*self)
    }
    /// Increase the lightness of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to increase the lightness by. Must be between 0.0 and 1.0.
    ///
    /// # Example
    /// ```
    /// use color_art::color;
    ///
    /// let mut color = color!(#80e619);
    /// color.lighten(0.2).unwrap();
    /// assert_eq!(color.hex(), "#b3f075");
    /// ```
    pub fn lighten(&mut self, amount: f64) -> Result<Self> {
        self.darken(-amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_darken() {
        let mut color = color!(#426105);
        color.darken(0.1).unwrap();
        assert_eq!(color.hex(), "#213102");

        let mut color = color!(#426105);
        color.darken(0.5).unwrap();
        assert_eq!(color.hex(), "#000000");

        let mut color = color!(#80e619);
        color.darken(0.2).unwrap();
        assert_eq!(color.hex(), "#4d8a0f");
    }
}
