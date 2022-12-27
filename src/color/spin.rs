use crate::{Color, ColorSpace};
use anyhow::Result;

impl Color {
    /// Rotate the hue angle of a color in either direction.
    pub fn spin(&self, angle: f64) -> Result<Color> {
        let color = self.space(ColorSpace::HSL)?;
        let h = color[0];
        let s = color[1];
        let l = color[2];
        let h = (h + angle) % 360.0;
        let h = if h < 0.0 { h + 360.0 } else { h };
        Color::from_hsl(h, s, l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_spin() {
        let color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
        let color = color.spin(30.0).unwrap();
        assert_eq!(color.hex(), "#f2a60d");

        let color = Color::from_str("hsl(10, 90%, 50%)").unwrap();
        let color = color.spin(-30.0).unwrap();
        assert_eq!(color.hex(), "#f20d59");
    }
}
