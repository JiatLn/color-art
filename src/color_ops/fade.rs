use crate::Color;

impl Color {
    /// Set the absolute opacity of a color.
    ///
    /// Can be applied to colors whether they already have an opacity value or not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::color;
    ///
    /// let color = color!(rgba(0, 255, 0, 0.8));
    /// assert_eq!(color.alpha(), 0.8);
    /// let color = color.fade(0.5);
    /// assert_eq!(color.alpha(), 0.5);
    /// ```
    pub fn fade(&self, amount: f64) -> Self {
        let alpha = amount.min(1.0).max(0.0);
        let (r, g, b) = self.rgb;
        Color::new(r, g, b, alpha)
    }
    /// Decrease the transparency (or increase the opacity) of a color, making it more opaque.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::color;
    ///
    /// let color = color!(rgba(0, 255, 0, 0.8));
    /// assert_eq!(color.alpha(), 0.8);
    /// let color = color.fade_in(0.1);
    /// assert_eq!(color.alpha(), 0.9);
    /// ```
    pub fn fade_in(&self, amount: f64) -> Self {
        let amount = (self.alpha + amount).min(1.0).max(0.0);
        self.fade(amount)
    }
    /// Increase the transparency (or decrease the opacity) of a color, making it less opaque.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::color;
    ///
    /// let color = color!(rgba(0, 255, 0, 0.8));
    /// assert_eq!(color.alpha(), 0.8);
    /// let color = color.fade_out(0.2);
    /// assert_eq!(color.alpha(), 0.6);
    /// ```
    pub fn fade_out(&self, amount: f64) -> Self {
        self.fade_in(-amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fade() {
        let color = color!(rgba(255, 0, 0, 1.0));
        let color = color.fade(0.5);
        assert_eq!(color.rgba(), "rgba(255, 0, 0, 0.5)");
    }

    #[test]
    fn test_fade_in() {
        let color = color!(rgba(255, 0, 0, 0.5));
        let color = color.fade_in(0.2);
        assert_eq!(color.rgba(), "rgba(255, 0, 0, 0.7)");
    }

    #[test]
    fn test_fade_out() {
        let color = color!(rgba(255, 0, 0, 0.5));
        let color = color.fade_out(0.2);
        assert_eq!(color.rgba(), "rgba(255, 0, 0, 0.3)");
    }
}
