use crate::{conversion, Color, ColorSpace};
use anyhow::{Ok, Result};

impl Color {
    /// Create a color from HSL values.
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Result<Color> {
        ColorSpace::HSL.valid(vec![h, s, l])?;
        let (r, g, b) = conversion::hsl::hsl2rgb((h, s, l));
        Ok(Color::new(r, g, b, 1.0))
    }
}
