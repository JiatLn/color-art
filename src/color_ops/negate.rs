use crate::Color;

impl Color {
    /// Negates a color with rgb channels. The alpha channel is not affected.
    ///
    /// # Example
    ///
    /// ```
    /// use color_art::color;
    ///
    /// let color1 = color!(#f0f);
    /// let color2 = color1.negate();
    /// assert_eq!(color2.hex(), "#0f0");
    /// ```
    pub fn negate(self) -> Self {
        Color::new(
            255.0 - self.rgb[0],
            255.0 - self.rgb[1],
            255.0 - self.rgb[2],
            self.alpha,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_negate() {
        let color = Color::from_rgb(0, 120, 255).unwrap();
        let negated_color = color.negate();
        assert_eq!(negated_color.rgb(), "rgb(255, 135, 0)");

        let color = Color::from_rgb(255, 255, 255).unwrap();
        let negated_color = color.negate();
        assert_eq!(negated_color.rgb(), "rgb(0, 0, 0)");

        let color = Color::from_rgb(0, 0, 0).unwrap();
        let negated_color = color.negate();
        assert_eq!(negated_color.rgb(), "rgb(255, 255, 255)");
    }
}
