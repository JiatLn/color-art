use crate::{conversion, Color, ColorSpace};

impl Color {
    /// Get the color space vector of the color instance.
    ///
    /// ⚗️ **Experimental**: This method is experimental and may change frequently in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::{color, ColorSpace};
    ///
    /// let color = color!(rgb(255, 51, 153));
    /// let vec = color.vec_of(ColorSpace::RGB);
    /// assert_eq!(vec, vec![255.0, 51.0, 153.0]);
    ///
    /// let vec = color.vec_of(ColorSpace::HSV);
    /// assert_eq!(vec, vec![330.0, 0.8, 1.0]);
    /// ```
    pub fn vec_of(&self, color_space: impl Into<ColorSpace>) -> Vec<f64> {
        let color = self.rgb.to_vec();
        let color_space = color_space.into();
        match color_space {
            ColorSpace::RGB | ColorSpace::HEX => color,
            ColorSpace::RGBA | ColorSpace::HEXA => {
                let [r, g, b] = self.rgb;
                vec![r, g, b, self.alpha]
            }
            ColorSpace::HSI => conversion::hsi::rgb2hsi(&color),
            ColorSpace::HSL => conversion::hsl::rgb2hsl(&color),
            ColorSpace::HSLA => {
                let mut hsl = conversion::hsl::rgb2hsl(&color);
                hsl.push(self.alpha);
                hsl
            }
            ColorSpace::HSV => conversion::hsv::rgb2hsv(&color),
            ColorSpace::HWB => conversion::hwb::rgb2hwb(&color),
            ColorSpace::CMYK => conversion::cmyk::rgb2cmyk(&color),
            ColorSpace::XYZ => conversion::xyz::rgb2xyz(&color),
            ColorSpace::YIQ => conversion::yiq::rgb2yiq(&color),
            ColorSpace::YUV => conversion::yuv::rgb2yuv(&color),
            ColorSpace::YCbCr => conversion::ycbcr::rgb2ycbcr(&color),
            ColorSpace::Lab => conversion::lab::rgb2lab(&color),
            ColorSpace::Unknown => todo!("Unknown color space not yet implemented `vec_of`"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils::round, *};

    #[test]
    fn test_vec_of_hsl() {
        let color = color!(#80e619);
        let vec = color.vec_of(ColorSpace::HSL);
        assert_eq!(vec, vec![89.85365853658537, 0.803921568627451, 0.5]);
    }

    #[test]
    fn test_vec_of_lab() {
        let color = color!(#7654cd);
        let vec = color
            .vec_of(ColorSpace::Lab)
            .iter()
            .map(|&v| round(v, 2))
            .collect::<Vec<_>>();
        assert_eq!(vec, vec![44.36, 36.05, -58.99]);
    }

    #[test]
    fn test_vec_of_xyz() {
        let color = color!(#7654cd);
        let vec = color
            .vec_of(ColorSpace::XYZ)
            .iter()
            .map(|&v| round(v, 5))
            .collect::<Vec<_>>();
        assert_eq!(vec, vec![0.2166, 0.146, 0.59437]);
    }

    #[test]
    fn test_vec_of_yiq() {
        let color = color!(#7654cd);
        let vec = color
            .vec_of(ColorSpace::YIQ)
            .iter()
            .map(|&v| round(v, 5))
            .collect::<Vec<_>>();
        assert_eq!(vec, vec![0.42337, -0.07301, 0.17583]);
    }
}
