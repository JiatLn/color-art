/// Color is a struct that represents a color
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub(crate) rgba: (f64, f64, f64, f64),
}
