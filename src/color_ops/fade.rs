use anyhow::{ bail, Ok, Result };

use crate::Color;

impl Color {
    /// Set the absolute opacity of a color.
    ///
    /// Can be applied to colors whether they already have an opacity value or not.
    ///
    /// # Examples
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("rgba(0, 255, 0, 0.8)").unwrap();
    /// assert_eq!(color.alpha(), 0.8);
    /// assert_eq!(color.fade(0.5).unwrap().alpha(), 0.5);
    /// ```
    pub fn fade(&mut self, amount: f64) -> Result<Self> {
        if amount < 0.0 || amount > 1.0 {
            bail!("Amount must be between 0.0 and 1.0");
        }
        self.alpha = amount;
        Ok(*self)
    }
    /// Decrease the transparency (or increase the opacity) of a color, making it more opaque.
    ///
    /// # Examples
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("rgba(0, 255, 0, 0.8)").unwrap();
    /// assert_eq!(color.alpha(), 0.8);
    /// assert_eq!(color.fade_in(0.1).unwrap().alpha(), 0.9);
    /// ```
    pub fn fade_in(&mut self, amount: f64) -> Result<Self> {
        if amount.abs() > 1.0 {
            bail!("Amount must be between 0.0 and 1.0");
        }
        let amount = (self.alpha + amount).min(1.0).max(0.0);
        self.fade(amount)
    }
    /// Increase the transparency (or decrease the opacity) of a color, making it less opaque.
    ///
    /// # Examples
    /// ```
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let mut color = Color::from_str("rgba(0, 255, 0, 0.8)").unwrap();
    /// assert_eq!(color.alpha(), 0.8);
    /// assert_eq!(color.fade_out(0.2).unwrap().alpha(), 0.6);
    /// ```
    pub fn fade_out(&mut self, amount: f64) -> Result<Self> {
        self.fade_in(-amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_fade() {
        let mut color = Color::from_str("rgb(255, 0, 0)").unwrap();
        color.fade(0.5).unwrap();
        assert_eq!(color.rgba(), "rgba(255, 0, 0, 0.5)");
    }
    #[test]
    fn test_fade_in() {
        let mut color = Color::from_str("rgba(255, 0, 0, 0.5)").unwrap();
        color.fade_in(0.2).unwrap();
        assert_eq!(color.rgba(), "rgba(255, 0, 0, 0.7)");
    }
    #[test]
    fn test_fade_out() {
        let mut color = Color::from_str("rgba(255, 0, 0, 0.5)").unwrap();
        color.fade_out(0.2).unwrap();
        assert_eq!(color.rgba(), "rgba(255, 0, 0, 0.3)");
    }
}
