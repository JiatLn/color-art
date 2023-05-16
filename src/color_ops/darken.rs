use crate::{ Color, ColorSpace };

impl Color {
    /// Decrease the lightness of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to decrease the lightness by. Must be between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::color;
    ///
    /// let color = color!(#426105);
    /// let color = color.darken(0.1);
    /// assert_eq!(color.hex(), "#213102");
    /// ```
    pub fn darken(&self, amount: f64) -> Self {
        let color = self.vec_of(ColorSpace::HSL);
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let l = (l - amount).min(1.0).max(0.0);
        Color::from_hsl(h, s, l).unwrap()
    }
    /// Increase the lightness of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to increase the lightness by. Must be between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::color;
    ///
    /// let color = color!(#80e619);
    /// let color = color.lighten(0.2);
    /// assert_eq!(color.hex(), "#b3f075");
    /// ```
    pub fn lighten(&self, amount: f64) -> Self {
        self.darken(-amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_darken() {
        let color = color!(#426105);
        let color = color.darken(0.1);
        assert_eq!(color.hex(), "#213102");

        let color = color!(#426105);
        let color = color.darken(0.5);
        assert_eq!(color.hex(), "#000");

        let color = color!(#80e619);
        let color = color.darken(0.2);
        assert_eq!(color.hex(), "#4d8a0f");
    }
}
