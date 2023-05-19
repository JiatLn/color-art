use crate::{ Color, ColorSpace };

impl Color {
    /// Increase the saturation of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to increase the saturation by. Must be between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::color;
    ///
    /// let color = color!(#80e619);
    /// let color = color.saturate(0.2);
    /// assert_eq!(color.hex(), "#80ff00");
    /// ```
    pub fn saturate(&self, amount: f64) -> Self {
        let color = self.vec_of(ColorSpace::HSL);
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let s = (s + amount).min(1.0).max(0.0);
        Color::from_hsl(h, s, l).unwrap()
    }
    /// Decrease the saturation of a color in the HSL color space by an absolute amount.
    ///
    /// # Arguments
    /// `amount` - The amount to decrease the saturation by. Must be between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::color;
    ///
    /// let color = color!(#80e619);
    /// let color = color.desaturate(0.2);
    /// assert_eq!(color.hex(), "#80cd32");
    /// ```
    pub fn desaturate(&self, amount: f64) -> Self {
        self.saturate(-amount)
    }
    /// greyscale
    ///
    /// Remove all saturation from a color in the HSL color space.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::color;
    /// use std::str::FromStr;
    ///
    /// let color = color!(#80e619);
    /// let color = color.greyscale();
    /// assert_eq!(color.hex(), "#808080");
    /// ```
    pub fn greyscale(&self) -> Self {
        self.desaturate(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn saturate() {
        let color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        let color = color.saturate(0.2);
        assert_eq!(color.hsl(), "hsl(60, 100%, 50%)");
    }

    #[test]
    fn desaturate() {
        let color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        let color = color.desaturate(0.2);
        assert_eq!(color.hsl(), "hsl(60, 60%, 50%)");
    }

    #[test]
    fn greyscale() {
        let color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
        let color = color.greyscale();
        assert_eq!(color.hex(), "#808080");

        let color = Color::from_str("hsl(90, 0%, 50%)").unwrap();
        let color = color.greyscale();
        assert_eq!(color.hex(), "#808080");

        let color = Color::from_str("hsl(0, 0%, 50%)").unwrap();
        let color = color.greyscale();
        assert_eq!(color.hex(), "#808080");
    }
}
