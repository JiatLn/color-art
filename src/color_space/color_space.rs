/// Color space enum.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorSpace {
    /// RGB color space.
    ///
    /// RGB stands for red, green, and blue.
    RGB,
    /// RGBA color space.
    ///
    /// RGBA stands for red, green, blue, and alpha.
    RGBA,
    /// HSI color space.
    ///
    /// HSI stands for hue, saturation, and intensity.
    HSI,
    /// HSL color space.
    ///
    /// HSL stands for hue, saturation, and lightness.
    HSL,
    /// HSLA color space.
    ///
    /// HSLA stands for hue, saturation, lightness, and alpha.
    HSLA,
    /// HSV color space.
    ///
    /// HSV stands for hue, saturation, and value.
    HSV,
    /// Hex color space.
    ///
    /// Hex stands for hexadecimal.
    HEX,
    /// HWB color space.
    ///
    /// HWB stands for hue, whiteness, and blackness.
    HWB,
    /// CMYK color space.
    ///
    /// CMYK means Cyan Magenta Yellow Black
    CMYK,
    /// XYZ color space.
    ///
    /// XYZ stands for X, Y, and Z.
    XYZ,
    /// [YIQ](https://en.wikipedia.org/wiki/YIQ) color space.
    ///
    /// YIQ stands for luminance (Y), and the chrominance components I and Q.
    YIQ,
    /// YUV color Space.
    ///
    /// YUV stands for luminance (Y), and the chrominance components U and V.
    YUV,
    /// YCbCr color Space.
    ///
    /// YCbCr stands for luminance (Y), and the chrominance components Cb and Cr.
    YCbCr,
    /// Lab color space.
    ///
    /// Lab stands for lightness, a, and b.
    Lab,
    /// Unknown color space.
    ///
    /// To be used when the color space is not known.
    Unknown,
}

impl ColorSpace {
    pub fn default() -> Self {
        ColorSpace::RGB
    }
}

impl<T> From<T> for ColorSpace
where
    T: ToString,
{
    fn from(s: T) -> Self {
        match s.to_string().to_lowercase().as_str() {
            "rgb" => ColorSpace::RGB,
            "rgba" => ColorSpace::RGBA,
            "hsi" => ColorSpace::HSI,
            "hsl" => ColorSpace::HSL,
            "hsla" => ColorSpace::HSLA,
            "hsv" => ColorSpace::HSV,
            "hex" => ColorSpace::HEX,
            "hwb" => ColorSpace::HWB,
            "cmyk" => ColorSpace::CMYK,
            "xyz" => ColorSpace::XYZ,
            "yiq" => ColorSpace::YIQ,
            "yuv" => ColorSpace::YUV,
            "ycbcr" => ColorSpace::YCbCr,
            "lab" => ColorSpace::Lab,
            _ => ColorSpace::Unknown,
        }
    }
}

impl ColorSpace {
    pub(crate) fn value_count(&self) -> usize {
        match self {
            ColorSpace::RGB => 3,
            ColorSpace::RGBA => 4,
            ColorSpace::HSI => 3,
            ColorSpace::HSL => 3,
            ColorSpace::HSLA => 4,
            ColorSpace::HSV => 3,
            ColorSpace::HEX => 3,
            ColorSpace::HWB => 3,
            ColorSpace::CMYK => 4,
            ColorSpace::XYZ => 3,
            ColorSpace::YIQ => 3,
            ColorSpace::YUV => 3,
            ColorSpace::YCbCr => 3,
            ColorSpace::Lab => 3,
            ColorSpace::Unknown => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_space_from_str() {
        assert_eq!(ColorSpace::from("rgb"), ColorSpace::RGB);
        assert_eq!(ColorSpace::from("rgba"), ColorSpace::RGBA);
        assert_eq!(ColorSpace::from("hsl"), ColorSpace::HSL);
        assert_eq!(ColorSpace::from("YCbCr"), ColorSpace::YCbCr);

        let rgb: ColorSpace = "rgb".into();
        assert_eq!(rgb, ColorSpace::RGB);

        let rgb: ColorSpace = "RGB".into();
        assert_eq!(rgb, ColorSpace::RGB);

        let rgba: ColorSpace = "rgba".into();
        assert_eq!(rgba, ColorSpace::RGBA);

        let rgba: ColorSpace = "RGBA".into();
        assert_eq!(rgba, ColorSpace::RGBA);
    }
}
