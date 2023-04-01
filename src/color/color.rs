/// Color is a struct that represents a color.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub(crate) rgb: (f64, f64, f64),
    pub(crate) alpha: f64,
}

impl Color {
    /// Creates a new [`Color`].
    pub fn new(r: f64, g: f64, b: f64, alpha: f64) -> Self {
        Color {
            rgb: (r, g, b),
            alpha,
        }
    }
}

impl Default for Color {
    /// default returns a black color.
    fn default() -> Self {
        Color {
            rgb: (0.0, 0.0, 0.0),
            alpha: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_default() {
        let color = Color::default();
        assert_eq!(color, Color::new(0.0, 0.0, 0.0, 1.0));
    }
}
