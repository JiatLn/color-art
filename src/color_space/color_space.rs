/// Color space enum.
#[derive(Clone)]
pub enum ColorSpace {
    /// RGB color space.
    ///
    /// RGB stands for red, green, and blue.
    RGB,
    /// RGBA color space.
    ///
    /// RGBA stands for red, green, blue, and alpha.
    RGBA,
    /// HSL color space.
    ///
    /// HSL stands for hue, saturation, and lightness.
    HSL,
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
    XYZ,
    /// YUV color Space.
    YUV,
    /// YCbCr color Space.
    YCbCr,
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
        match s.to_string().as_str() {
            "rgb" => ColorSpace::RGB,
            "rgba" => ColorSpace::RGBA,
            "hsl" => ColorSpace::HSL,
            "hsv" => ColorSpace::HSV,
            "hex" => ColorSpace::HEX,
            "hwb" => ColorSpace::HWB,
            "cmyk" => ColorSpace::CMYK,
            "xyz" => ColorSpace::XYZ,
            "yuv" => ColorSpace::YUV,
            "YCbCr" => ColorSpace::YCbCr,
            _ => ColorSpace::Unknown,
        }
    }
}
