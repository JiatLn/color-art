use anyhow::{ anyhow, Result };

use crate::Color;

impl Color {
    /// Returns the numeric representation of the hexadecimal color.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_num(0xff3399).unwrap();
    /// assert_eq!(color.hex(), "#f39");
    /// ```
    pub fn from_num(num: u32) -> Result<Self> {
        if num > 0xffffff {
            return Err(anyhow!("Invalid color number, must be between 0 and 16777215"));
        }
        let r = ((num >> 16) & 0xff) as f64;
        let g = ((num >> 8) & 0xff) as f64;
        let b = (num & 0xff) as f64;
        Ok(Color::new(r, g, b, 1.0))
    }
}

#[test]
fn test_color_from_num() {
    let color = Color::from_num(0xff3399).unwrap();
    assert_eq!(color.hex(), "#f39");

    let color = Color::from_num(777).unwrap();
    assert_eq!(color.hex(), "#000309");

    let color = Color::from_num(0x1000000);
    assert!(color.is_err());
}
