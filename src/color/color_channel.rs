use crate::utils::*;
use crate::{Color, ColorSpace};

/// Color channel extraction methods.
///
/// # Examples
/// ```
/// use color_art::{Color, color};
/// use std::str::FromStr;
///
/// let color = color!(rgba(10, 20, 30, 0.8));
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
    /// Extracts the red channel of color as a number between 0 and 255.
    pub fn red(&self) -> u8 {
        round(self.rgb[0], 0) as u8
    }
    /// Extracts the green channel of color as a number between 0 and 255.
    pub fn green(&self) -> u8 {
        round(self.rgb[1], 0) as u8
    }
    /// Extracts the blue channel of color as a number between 0 and 255.
    pub fn blue(&self) -> u8 {
        round(self.rgb[2], 0) as u8
    }
    /// Extracts the alpha channel of color as a number between 0.0 and 1.0.
    pub fn alpha(&self) -> f64 {
        round(self.alpha, 2)
    }
    /// Extracts the hue channel of color as a number between 0.0 and 360.0.
    pub fn hue(&self) -> f64 {
        self.vec_of(ColorSpace::HSL)[0]
    }
    /// Extracts the HSL saturation of color as a number between 0.0 and 1.0.
    pub fn saturation(&self) -> f64 {
        self.vec_of(ColorSpace::HSL)[1]
    }
    /// Extracts the HSL lightness of color as a number between 0.0 and 1.0.
    pub fn lightness(&self) -> f64 {
        self.vec_of(ColorSpace::HSL)[2]
    }
    /// Extracts the HWB whiteness of color as a number between 0.0 and 1.0.
    pub fn whiteness(&self) -> f64 {
        self.vec_of(ColorSpace::HWB)[1]
    }
    /// Extracts the HWB blackness of color as a number between 0.0 and 1.0.
    pub fn blackness(&self) -> f64 {
        self.vec_of(ColorSpace::HWB)[2]
    }
    /// Calculates the [relative luminance](https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef) of color.
    ///
    /// the relative brightness of any point in a colorspace, normalized to 0 for darkest black and 1 for lightest white.
    ///
    /// same as `luminance()`
    pub fn luma(&self) -> f64 {
        self.luminance()
    }
    /// Calculates the [relative luminance](https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef) of color.
    ///
    /// the relative brightness of any point in a colorspace, normalized to 0 for darkest black and 1 for lightest white.
    ///
    /// same as `luma()`
    pub fn luminance(&self) -> f64 {
        let color = normalize_color(&self.rgb);
        let r = luminance_x(color[0]);
        let g = luminance_x(color[1]);
        let b = luminance_x(color[2]);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
    /// Extracts the hue channel of color in the HSV color space.
    pub fn hsv_hue(&self) -> f64 {
        self.vec_of(ColorSpace::HSV)[0]
    }
    /// Extracts the saturation channel of color in the HSV color space.
    pub fn hsv_saturation(&self) -> f64 {
        self.vec_of(ColorSpace::HSV)[1]
    }
    /// Extracts the value channel of color in the HSV color space.
    pub fn hsv_value(&self) -> f64 {
        self.vec_of(ColorSpace::HSV)[2]
    }
    /// Calculates the [gray](http://en.wikipedia.org/wiki/Grayscale) value of color.
    pub fn gray(&self) -> f64 {
        let [r, g, b] = self.rgb;
        0.299 * r + 0.587 * g + 0.114 * b
    }
}

fn luminance_x(x: f64) -> f64 {
    if x <= 0.03928 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{assert_eq, str::FromStr};

    #[test]
    fn test_color_channel() {
        let color = color!(rgba(10, 20, 30, 0.8));

        assert_eq!(color.red(), 10);
        assert_eq!(color.green(), 20);
        assert_eq!(color.blue(), 30);
        assert_eq!(color.alpha(), 0.8);

        assert_eq!(color.whiteness(), 0.0392156862745098);
        assert_eq!(color.blackness(), 0.8823529411764706);

        assert_eq!(color.luma(), 0.006585790668061925);
        assert_eq!(color.luminance(), 0.006585790668061925);
        assert_eq!(color.gray(), 18.15);

        let color = Color::from_str("hsl(90, 100%, 50%)").unwrap();

        assert_eq!(color.hue(), 90.0);
        assert_eq!(color.saturation(), 1.0);
        assert_eq!(color.lightness(), 0.5);

        assert_eq!(color.whiteness(), 0.0);
        assert_eq!(color.blackness(), 0.0);

        assert_eq!(color.luma(), 0.7607051464665225);
        assert_eq!(color.luminance(), 0.7607051464665225);
        assert_eq!(color.gray(), 187.8075);

        let color = Color::from_str("hsv(90, 100%, 50%)").unwrap();

        assert_eq!(color.hsv_hue(), 90.0);
        assert_eq!(color.hsv_saturation(), 1.0);
        assert_eq!(color.hsv_value(), 0.5);
        assert_eq!(color.gray(), 93.90375);

        let color = color!(rgb(100, 200, 30));

        assert_eq!(color.luma(), 0.44111615679100963);
        assert_eq!(color.luminance(), 0.44111615679100963);
        assert_eq!(color.gray(), 150.71999999999997);
    }
}
