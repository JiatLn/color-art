use crate::Color;

impl Color {
    /// Mix two colors with a weight.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to mix with.
    /// * `weight` - The weight of the new color to mix with. 0.0 is all the original color, 1.0 is all the new color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::color;
    ///
    /// let color1 = color!(#998099);
    /// let color2 = color!(#d2e1dd);
    /// let color3 = color1.mix_with(&color2, 0.5);
    /// assert_eq!(color3.hex(), "#b6b1bb");
    /// ```
    pub fn mix_with(&self, new_color: &Color, weight: f64) -> Self {
        let weight = weight.min(1.0).max(0.0);
        let old_weight = 1.0 - weight;

        let rgb1 = self.rgb;
        let rgb2 = new_color.rgb;

        let r = rgb1[0] * old_weight + rgb2[0] * weight;
        let g = rgb1[1] * old_weight + rgb2[1] * weight;
        let b = rgb1[2] * old_weight + rgb2[2] * weight;
        let alpha = self.alpha * old_weight + new_color.alpha * weight;

        Color::new(r, g, b, alpha)
    }
    /// Mix color with white in variable proportion.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of white to mix in. 0.0 is no white, 1.0 is all white.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::color;
    ///
    /// let color = color!(#ff00ff);
    /// let color = color.tint(0.5);
    /// assert_eq!(color.hex(), "#ff80ff");
    /// ```
    pub fn tint(&self, amount: f64) -> Self {
        let white = Color::new(255.0, 255.0, 255.0, 1.0);
        self.mix_with(&white, 1.0 - amount)
    }
    /// Mix color with black in variable proportion.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of black to mix in. 0.0 is no black, 1.0 is all black.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::color;
    ///
    /// let color = color!(#ff00ff);
    /// let color = color.shade(0.5);
    /// assert_eq!(color.hex(), "#800080");
    /// ```
    pub fn shade(&self, amount: f64) -> Self {
        let black = Color::default();
        self.mix_with(&black, 1.0 - amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_mix() {
        let color1 = color!(#003366);
        let color2 = color!(#d2e1dd);
        let color3 = color1.mix_with(&color2, 0.5);
        assert_eq!(color3.hex(), "#698aa2");

        let color1 = color!(#ff0000);
        let color2 = color!(#0000ff);
        let color3 = color1.mix_with(&color2, 0.5);
        assert_eq!(color3.hex(), "#800080");
    }

    #[test]
    fn test_tint() {
        let color = color!(rgba(0, 0, 255, 0.5));
        let color = color.tint(0.5);
        assert_eq!(color.rgba(), "rgba(128, 128, 255, 0.75)");

        let color = color!(rgb(255, 0, 0));
        let color = color.tint(0.5);
        assert_eq!(color.hex(), "#ff8080");
    }

    #[test]
    fn test_shade() {
        let color = color!(rgba(0, 0, 255, 0.5));
        let color = color.shade(0.5);
        assert_eq!(color.rgba(), "rgba(0, 0, 128, 0.75)");

        let color = color!(rgb(255, 0, 0));
        let color = color.shade(0.5);
        assert_eq!(color.hex(), "#800000");
    }
}
