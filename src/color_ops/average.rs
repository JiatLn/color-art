use crate::Color;

impl Color {
    /// Average a list of colors.
    ///
    /// This function will return a new color that is the average of the colors
    /// in the list.
    /// It will calculate the average of the RGB channels and alpha values of the colors.
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let colors = vec![
    ///    Color::from_str("#ff6600").unwrap(),
    ///    Color::from_str("rgba(0, 0, 0, 0.5)").unwrap(),
    /// ];
    ///
    /// let averaged_color = Color::average(&colors);
    /// assert_eq!(averaged_color.rgba(), "rgba(128, 51, 0, 0.75)");
    /// ```
    pub fn average(colors: &Vec<Color>) -> Color {
        let len = colors.len() as f64;

        if len == 0.0 {
            return Color::new(0.0, 0.0, 0.0, 0.0);
        }

        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        let mut a = 0.0;

        for color in colors {
            r += color.rgba.0;
            g += color.rgba.1;
            b += color.rgba.2;
            a += color.rgba.3;
        }

        Color::new(r / len, g / len, b / len, a / len)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_average_colors() {
        let colors = vec![
            Color::from_str("red").unwrap(),
            Color::from_str("rgba(0, 0, 0, 0.5)").unwrap(),
        ];

        let averaged_color = Color::average(&colors);
        assert_eq!(averaged_color.rgba(), "rgba(128, 0, 0, 0.75)");

        let colors = vec![
            Color::from_str("#ff6600").unwrap(),
            Color::from_str("#0000ff").unwrap(),
        ];
        let averaged_color = Color::average(&colors);
        assert_eq!(averaged_color.hex(), "#803380");

        let colors = vec![
            Color::from_str("#ffff00").unwrap(),
            Color::from_str("#ff0000").unwrap(),
            Color::from_str("#0000ff").unwrap(),
        ];
        let averaged_color = Color::average(&colors);
        assert_eq!(averaged_color.hex(), "#aa5555");
    }
}
