use crate::{conversion, Color, ColorSpace};

impl Color {
    /// Get the color space vector of the color instance.
    ///
    /// ⚗️ **Experimental**: This method is experimental and may change or be removed in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::{Color, ColorSpace};
    ///
    /// let color = Color::from_rgb(255.0, 51.0, 153.0).unwrap();
    /// let vec = color.vec_of(ColorSpace::RGB);
    /// assert_eq!(vec, vec![255.0, 51.0, 153.0]);
    ///
    /// let vec = color.vec_of(ColorSpace::HSV);
    /// assert_eq!(vec, vec![330.0, 0.8, 1.0]);
    /// ```
    pub fn vec_of(&self, color_space: ColorSpace) -> Vec<f64> {
        let color = self.rgb;
        match color_space {
            ColorSpace::RGB => {
                let (r, g, b) = color;
                vec![r, g, b]
            }
            ColorSpace::RGBA => {
                let (r, g, b) = color;
                vec![r, g, b, self.alpha]
            }
            ColorSpace::HSL => {
                let (h, s, l) = conversion::hsl::rgb2hsl(color);
                vec![h, s, l]
            }
            ColorSpace::HSLA => {
                let (h, s, l) = conversion::hsl::rgb2hsl(color);
                vec![h, s, l, self.alpha]
            }
            ColorSpace::HSV => {
                let (h, s, v) = conversion::hsv::rgb2hsv(color);
                vec![h, s, v]
            }
            ColorSpace::HWB => {
                let (h, w, b) = conversion::hwb::rgb2hwb(color);
                vec![h, w, b]
            }
            ColorSpace::CMYK => {
                let (c, m, y, k) = conversion::cmyk::rgb2cmyk(color);
                vec![c, m, y, k]
            }
            ColorSpace::XYZ => {
                let (x, y, z) = conversion::xyz::rgb2xyz(color);
                vec![x, y, z]
            }
            ColorSpace::YUV => {
                let (y, u, v) = conversion::yuv::rgb2yuv(color);
                vec![y, u, v]
            }
            ColorSpace::YCbCr => {
                let (y, cb, cr) = conversion::ycbcr::rgb2ycbcr(color);
                vec![y, cb, cr]
            }
            ColorSpace::Lab => {
                let (l, a, b) = conversion::lab::rgb2lab(color);
                vec![l, a, b]
            }
            ColorSpace::HEX | ColorSpace::Unknown => {
                todo!("not implemented yet")
            }
        }
    }
}
