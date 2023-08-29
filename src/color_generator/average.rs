use crate::Color;

impl Color {
    /// Average a list of colors.
    ///
    /// This function will return a new color that is the average of the colors
    /// in the list.
    /// It will calculate the average of the RGB channels and alpha values of the colors.
    /// If the list length is 0, it will return a black color.
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
    pub fn average(colors: &[Color]) -> Color {
        if colors.is_empty() {
            return Color::default();
        }

        let vec = colors
            .iter()
            .fold([0.0, 0.0, 0.0, 0.0], |acc, color| {
                let [r, g, b] = color.rgb;
                let a = color.alpha;
                [acc[0] + r, acc[1] + g, acc[2] + b, acc[3] + a]
            })
            .iter()
            .map(|v| v / (colors.len() as f64))
            .collect::<Vec<f64>>();

        Color::new(vec[0], vec[1], vec[2], vec[3])
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::str::FromStr;

    #[test]
    fn test_average_colors() {
        let colors = vec![Color::from_str("red").unwrap(), color!(rgba(0, 0, 0, 0.5))];

        let averaged_color = Color::average(&colors);
        assert_eq!(averaged_color.rgba(), "rgba(128, 0, 0, 0.75)");

        let colors = vec![color!(#ff6600), color!(#0000ff)];
        let averaged_color = Color::average(&colors);
        assert_eq!(averaged_color.hex(), "#803380");

        let colors = vec![color!(#ffff00), color!(#ff0000), color!(#0000ff)];
        let averaged_color = Color::average(&colors);
        assert_eq!(averaged_color.hex(), "#a55");
    }

    #[test]
    fn test_average_empty_list() {
        let averaged_color = Color::average(&vec![]);
        assert_eq!(averaged_color.rgba(), "rgba(0, 0, 0, 1)");
    }
}
