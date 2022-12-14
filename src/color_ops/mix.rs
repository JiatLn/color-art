use crate::Color;
use anyhow::{bail, Ok, Result};

impl Color {
    /// Mix two colors with a weight.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to mix with.
    /// * `weight` - The weight of the color to mix with. 0.0 is all the original color, 1.0 is all the new color.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#998099").unwrap();
    /// let another_color = Color::from_str("#d2e1dd").unwrap();
    /// color.mix_with(&another_color, 0.5).unwrap();
    /// assert_eq!(color.hex(), "#b6b1bb");
    /// ```
    pub fn mix_with(&mut self, color: &Color, weight: f64) -> Result<Self> {
        if weight < 0.0 || weight > 1.0 {
            bail!("weight must be between 0.0 and 1.0");
        }
        let rgba1 = self.rgba;
        let rgba2 = color.rgba;
        let w1 = weight;
        let w2 = 1.0 - weight;
        self.rgba.0 = rgba1.0 * w1 + rgba2.0 * w2;
        self.rgba.1 = rgba1.1 * w1 + rgba2.1 * w2;
        self.rgba.2 = rgba1.2 * w1 + rgba2.2 * w2;
        self.rgba.3 = rgba1.3 * w1 + rgba2.3 * w2;
        Ok(*self)
    }
    /// Mix color with white in variable proportion.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of white to mix in. 0.0 is no white, 1.0 is all white.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#ff00ff").unwrap();
    /// color.tint(0.5).unwrap();
    /// assert_eq!(color.hex(), "#ff80ff");
    /// ```
    pub fn tint(&mut self, amount: f64) -> Result<Self> {
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
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#ff00ff").unwrap();
    /// color.shade(0.5).unwrap();
    /// assert_eq!(color.hex(), "#800080");
    /// ```
    pub fn shade(&mut self, amount: f64) -> Result<Self> {
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        self.mix_with(&black, 1.0 - amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_mix() {
        let mut color1 = Color::from_str("#003366").unwrap();
        let color2 = Color::from_str("#d2e1dd").unwrap();
        color1.mix_with(&color2, 0.5).unwrap();
        assert_eq!(color1.hex(), "#698aa2");

        let mut color3 = Color::from_str("#ff0000").unwrap();
        let color4 = Color::from_str("#0000ff").unwrap();
        color3.mix_with(&color4, 0.5).unwrap();
        assert_eq!(color3.hex(), "#800080");
    }

    #[test]
    fn test_tint() {
        let mut color = Color::from_str("rgba(0,0,255,0.5)").unwrap();
        color.tint(0.5).unwrap();
        assert_eq!(color.rgba(), "rgba(128, 128, 255, 0.75)");

        let mut color = Color::from_str("red").unwrap();
        color.tint(0.5).unwrap();
        assert_eq!(color.hex(), "#ff8080");
    }

    #[test]
    fn test_shade() {
        let mut color = Color::from_str("rgba(0,0,255,0.5)").unwrap();
        color.shade(0.5).unwrap();
        assert_eq!(color.rgba(), "rgba(0, 0, 128, 0.75)");

        let mut color = Color::from_str("red").unwrap();
        color.shade(0.5).unwrap();
        assert_eq!(color.hex(), "#800000");
    }
}
