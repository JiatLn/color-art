use crate::Color;
use std::str::FromStr;

static DIGITS: &str = "0123456789abcdef";

impl Color {
    /// Generate a random color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::random();
    /// ```
    pub fn random() -> Color {
        let mut color = String::from("#");
        for _ in 0..6 {
            let index = rand::random::<usize>() % DIGITS.len();
            color.push(DIGITS.chars().nth(index).unwrap());
        }
        Color::from_str(&color).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        for _ in 0..100 {
            let color = Color::random();
            assert!(color.rgb[0] >= 0.0 && color.rgb[0] <= 255.0);
            assert!(color.rgb[1] >= 0.0 && color.rgb[1] <= 255.0);
            assert!(color.rgb[2] >= 0.0 && color.rgb[2] <= 255.0);
            assert!(color.alpha >= 0.0 && color.alpha <= 1.0);
        }
    }
}
