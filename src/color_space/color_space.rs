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
            "yuv" => ColorSpace::YUV,
            "ycbcr" => ColorSpace::YCbCr,
            "lab" => ColorSpace::Lab,
            _ => ColorSpace::Unknown,
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

        let rgba: ColorSpace = "rgba".into();
        assert_eq!(rgba, ColorSpace::RGBA);
    }
}
