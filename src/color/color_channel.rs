use crate::{helper::*, Color, ColorSpace};

/// Color channel extraction methods.
///
/// # Examples
/// ```
/// use color_art::Color;
/// use std::str::FromStr;
///
/// let color = Color::from_str("rgba(10, 20, 30, 0.8)").unwrap();
/// assert_eq!(color.red(), 10);
/// assert_eq!(color.green(), 20);
/// assert_eq!(color.blue(), 30);
/// assert_eq!(color.alpha(), 0.8);
///
/// let color = Color::from_str("hsl(90, 100%, 50%)").unwrap();
/// assert_eq!(color.hue(), 90.0);
/// assert_eq!(color.saturation(), 1.0);
/// assert_eq!(color.lightness(), 0.5);
/// ```
impl Color {
    /// Extracts the red channel of a color in the RGB color space.
    pub fn red(&self) -> u8 {
        round(self.rgba.0, 0) as u8
    }
    /// Extracts the green channel of a color in the RGB color space.
    pub fn green(&self) -> u8 {
        round(self.rgba.1, 0) as u8
    }
    /// Extracts the blue channel of a color in the RGB color space.
    pub fn blue(&self) -> u8 {
        round(self.rgba.2, 0) as u8
    }
    /// Extracts the alpha channel of a color in the RGBA color space.
    pub fn alpha(&self) -> f64 {
        round(self.rgba.3, 1)
    }
    /// Extracts the hue channel of a color in the HSL color space.
    pub fn hue(&self) -> f64 {
        match self.space(ColorSpace::HSL) {
            Ok(hsl) => hsl[0],
            Err(_) => 0.0,
        }
    }
    /// Extracts the saturation channel of a color in the HSL color space.
    pub fn saturation(&self) -> f64 {
        match self.space(ColorSpace::HSL) {
            Ok(hsl) => hsl[1],
            Err(_) => 0.0,
        }
    }
    /// Extracts the lightness channel of a color in the HSL color space.
    pub fn lightness(&self) -> f64 {
        match self.space(ColorSpace::HSL) {
            Ok(hsl) => hsl[2],
            Err(_) => 0.0,
        }
    }
    /// Calculates the [luma](http://en.wikipedia.org/wiki/Luma_%28video%29) (perceptual brightness) of a color.
    pub fn luma(&self) -> f64 {
        let (r, g, b, a) = self.rgba;
        let (r, g, b) = normalize_color((r, g, b));

        let r = if r <= 0.03928 {
            r / 12.92
        } else {
            ((r + 0.055) / 1.055).powf(2.4)
        };

        let g = if g <= 0.03928 {
            g / 12.92
        } else {
            ((g + 0.055) / 1.055).powf(2.4)
        };

        let b = if b <= 0.03928 {
            b / 12.92
        } else {
            ((b + 0.055) / 1.055).powf(2.4)
        };

        round(0.2126 * r + 0.7152 * g + 0.0722 * b * a, 2)
    }
    /// Calculates the value of the luma without gamma correction.
    pub fn luminance(&self) -> f64 {
        let (r, g, b, a) = self.rgba;
        let (r, g, b) = normalize_color((r, g, b));
        round(0.2126 * r + 0.7152 * g + 0.0722 * b * a, 2)
    }
    /// Extracts the hue channel of a color in the HSV color space.
    pub fn hsv_hue(&self) -> f64 {
        match self.space(ColorSpace::HSV) {
            Ok(hsv) => hsv[0],
            Err(_) => 0.0,
        }
    }
    /// Extracts the saturation channel of a color in the HSV color space.
    pub fn hsv_saturation(&self) -> f64 {
        match self.space(ColorSpace::HSV) {
            Ok(hsv) => hsv[1],
            Err(_) => 0.0,
        }
    }
    /// Extracts the value channel of a color in the HSV color space.
    pub fn hsv_value(&self) -> f64 {
        match self.space(ColorSpace::HSV) {
            Ok(hsv) => hsv[2],
            Err(_) => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_color_channel() {
        let color = Color::from_str("rgba(10, 20, 30, 0.8)").unwrap();

        assert_eq!(color.red(), 10);
        assert_eq!(color.green(), 20);
        assert_eq!(color.blue(), 30);
        assert_eq!(color.alpha(), 0.8);

        assert_eq!(color.luma(), 0.01);
        assert_eq!(color.luminance(), 0.07);

        let color = Color::from_str("hsl(90, 100%, 50%)").unwrap();

        assert_eq!(color.hue(), 90.0);
        assert_eq!(color.saturation(), 1.0);
        assert_eq!(color.lightness(), 0.5);

        assert_eq!(color.luma(), 0.76);
        assert_eq!(color.luminance(), 0.82);

        let color = Color::from_str("hsv(90, 100%, 50%)").unwrap();

        assert_eq!(color.hsv_hue(), 90.0);
        assert_eq!(color.hsv_saturation(), 1.0);
        assert_eq!(color.hsv_value(), 0.5);

        let color = Color::from_str("rgb(100, 200, 30)").unwrap();

        assert_eq!(color.luma(), 0.44);
        assert_eq!(color.luminance(), 0.65);
    }
}
