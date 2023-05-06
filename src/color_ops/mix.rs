use crate::Color;
use anyhow::{bail, Ok, Result};

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
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("#998099").unwrap();
    /// let another_color = Color::from_str("#d2e1dd").unwrap();
    /// color.mix_with(&another_color, 0.5).unwrap();
    /// assert_eq!(color.hex(), "#b6b1bb");
    /// ```
    pub fn mix_with(&mut self, new_color: &Color, weight: f64) -> Result<Self> {
        if weight < 0.0 || weight > 1.0 {
            bail!("weight must be between 0.0 and 1.0");
        }
        let rgb1 = self.rgb;
        let rgb2 = new_color.rgb;
        let old_weight = 1.0 - weight;
        self.rgb = (
            rgb1.0 * old_weight + rgb2.0 * weight,
            rgb1.1 * old_weight + rgb2.1 * weight,
            rgb1.2 * old_weight + rgb2.2 * weight,
        );
        self.alpha = self.alpha * old_weight + new_color.alpha * weight;
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
        let black = Color::default();
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
