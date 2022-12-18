use crate::{conversion::hex::rgb2hex, Color};

/// Stringify a color to a string.
impl Color {
    /// `hex` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hex(), "#ffffff");
    /// ```
    pub fn hex(self) -> String {
        let (r, g, b, _) = self.rgba;
        rgb2hex((r, g, b))
    }
    /// `rgb` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.rgb(), "rgb(255, 255, 255)");
    /// ```
    pub fn rgb(self) -> String {
        let (r, g, b, _) = self.rgba;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgb({}, {}, {})", r, g, b)
    }
    /// `rgba` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 0.5);
    /// assert_eq!(color.rgba(), "rgba(255, 255, 255, 0.5)");
    /// ```
    pub fn rgba(self) -> String {
        let (r, g, b, a) = self.rgba;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgba({}, {}, {}, {})", r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let color = Color::new(255.0, 255.0, 255.0, 1.0);
        assert_eq!(color.hex(), "#ffffff");

        let color = Color::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(color.hex(), "#000000");

        let color = Color::new(0.0, 128.0, 128.4, 1.0);
        assert_eq!(color.hex(), "#008080");
    }

    #[test]
    fn test_rgb() {
        let color = Color::new(255.0, 255.0, 255.0, 1.0);
        assert_eq!(color.rgb(), "rgb(255, 255, 255)");

        let color = Color::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(color.rgb(), "rgb(0, 0, 0)");

        let color = Color::new(0.0, 128.0, 128.4, 1.0);
        assert_eq!(color.rgb(), "rgb(0, 128, 128)");
    }

    #[test]
    fn test_rgba() {
        let color = Color::new(255.0, 255.0, 255.0, 0.5);
        assert_eq!(color.rgba(), "rgba(255, 255, 255, 0.5)");

        let color = Color::new(0.0, 0.0, 0.0, 0.5);
        assert_eq!(color.rgba(), "rgba(0, 0, 0, 0.5)");

        let color = Color::new(0.0, 128.0, 128.4, 0.5);
        assert_eq!(color.rgba(), "rgba(0, 128, 128, 0.5)");
    }
}
