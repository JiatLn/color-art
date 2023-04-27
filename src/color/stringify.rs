use crate::{
    conversion::{
        cmyk::rgb2cmyk,
        hex::{rgb2hex, rgba2hex},
        hsl::rgb2hsl,
        hsv::rgb2hsv,
        hwb::rgb2hwb,
        lab::rgb2lab,
        xyz::rgb2xyz,
        ycbcr::rgb2ycbcr,
        yuv::rgb2yuv,
    },
    data::name_of_hex,
    helper::round,
    Color,
};

/// Stringify a color to a string.
impl Color {
    /// `hex` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255, 255, 255, 1.0);
    /// assert_eq!(color.hex(), "#ffffff");
    ///
    /// let color = Color::new(255, 255, 255, 0.5);
    /// assert_eq!(color.hex(), "#ffffff80");
    /// ```
    pub fn hex(self) -> String {
        if self.alpha == 1.0 {
            rgb2hex(self.rgb)
        } else {
            let (r, g, b) = self.rgb;
            let color = (r, g, b, self.alpha);
            rgba2hex(color)
        }
    }
    /// `rgb` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.rgb(), "rgb(255, 255, 255)");
    /// ```
    pub fn rgb(self) -> String {
        let (r, g, b) = self.rgb;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgb({}, {}, {})", r, g, b)
    }
    /// `rgba` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 0.5);
    /// assert_eq!(color.rgba(), "rgba(255, 255, 255, 0.5)");
    /// ```
    pub fn rgba(self) -> String {
        let (r, g, b) = self.rgb;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgba({}, {}, {}, {})", r, g, b, self.alpha)
    }
    /// `hsl` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hsl(), "hsl(0, 0%, 100%)");
    /// ```
    pub fn hsl(self) -> String {
        let (h, s, l) = rgb2hsl(self.rgb);
        format!(
            "hsl({}, {}%, {}%)",
            round(h, 0),
            round(s * 100., 0),
            round(l * 100., 0),
        )
    }
    /// `hsv` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hsv(), "hsv(0, 0%, 100%)");
    /// ```
    pub fn hsv(self) -> String {
        let (h, s, v) = rgb2hsv(self.rgb);
        format!(
            "hsv({}, {}%, {}%)",
            round(h, 0),
            round(s * 100., 0),
            round(v * 100., 0),
        )
    }
    /// `hwb` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hwb(), "hwb(0, 100%, 0%)");
    /// ```
    pub fn hwb(self) -> String {
        let (h, w, b) = rgb2hwb(self.rgb);
        format!(
            "hwb({}, {}%, {}%)",
            round(h, 0),
            round(w * 100., 0),
            round(b * 100., 0),
        )
    }
    /// `cmyk` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.cmyk(), "cmyk(0%, 0%, 0%, 0%)");
    /// ```
    pub fn cmyk(self) -> String {
        let (c, m, y, k) = rgb2cmyk(self.rgb);
        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            round(c * 100., 0),
            round(m * 100., 0),
            round(y * 100., 0),
            round(k * 100., 0),
        )
    }
    /// `xyz` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 0.0, 0.0, 1.0);
    /// assert_eq!(color.xyz(), "xyz(0.757088, 0.596903, 0.260887)");
    /// ```
    pub fn xyz(self) -> String {
        let (x, y, z) = rgb2xyz(self.rgb);
        format!("xyz({}, {}, {})", round(x, 6), round(y, 6), round(z, 6))
    }
    /// `yuv` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 0.0, 0.0, 1.0);
    /// assert_eq!(color.yuv(), "yuv(0.299, -0.1471, 0.6148)");
    /// ```
    pub fn yuv(self) -> String {
        let (y, u, v) = rgb2yuv(self.rgb);
        format!("yuv({}, {}, {})", round(y, 4), round(u, 4), round(v, 4))
    }
    /// `lab` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 0.0, 1.0);
    /// assert_eq!(color.lab(), "lab(97.14, -21.55, 94.48)");
    /// ```
    pub fn lab(self) -> String {
        let (l, a, b) = rgb2lab(self.rgb);
        format!("lab({}, {}, {})", round(l, 2), round(a, 2), round(b, 2))
    }
    /// `YCbCr` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 0.0, 1.0);
    /// assert_eq!(color.ycbcr(), "YCbCr(225.93, 0.5755, 148.7269)");
    /// ```
    pub fn ycbcr(self) -> String {
        let (y, cb, cr) = rgb2ycbcr(self.rgb);
        format!("YCbCr({}, {}, {})", round(y, 4), round(cb, 4), round(cr, 4))
    }
    /// `name` of the color
    ///
    /// The color name is based on the [CSS3 color name](https://www.w3.org/TR/css-color-3/#svg-color) or 中国传统色彩.
    ///
    /// If the color is not named, the hex string will be returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::from_hex("#ffffff").unwrap();
    /// assert_eq!(color.name(), "white");
    ///
    /// let color = Color::from_hex("#f8df72").unwrap();
    /// assert_eq!(color.name(), "茉莉黄");
    ///
    /// let color = Color::new(42, 42, 42, 1.0);
    /// assert_eq!(color.name(), "#2a2a2a");
    /// ```
    pub fn name(self) -> String {
        let hex = self.hex();

        match name_of_hex(&hex) {
            Some(name) => name.to_string(),
            None => hex,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify_color() {
        let color = Color::new(255.0, 255.0, 255.0, 1.0);
        assert_eq!(color.hex(), "#ffffff");
        assert_eq!(color.rgb(), "rgb(255, 255, 255)");
        assert_eq!(color.rgba(), "rgba(255, 255, 255, 1)");
        assert_eq!(color.hsl(), "hsl(0, 0%, 100%)");
        assert_eq!(color.hsv(), "hsv(0, 0%, 100%)");
        assert_eq!(color.hwb(), "hwb(0, 100%, 0%)");
        assert_eq!(color.xyz(), "xyz(1, 1, 1)");
        assert_eq!(color.ycbcr(), "YCbCr(255, 128, 128)");
        assert_eq!(color.lab(), "lab(100, 0, 0)");
        assert_eq!(color.name(), "white");

        let color = Color::new(0.0, 0.0, 0.0, 0.5);
        assert_eq!(color.hex(), "#00000080");
        assert_eq!(color.rgb(), "rgb(0, 0, 0)");
        assert_eq!(color.rgba(), "rgba(0, 0, 0, 0.5)");
        assert_eq!(color.hsl(), "hsl(0, 0%, 0%)");
        assert_eq!(color.hsv(), "hsv(0, 0%, 0%)");
        assert_eq!(color.hwb(), "hwb(0, 0%, 100%)");
        assert_eq!(color.xyz(), "xyz(0.137931, 0.137931, 0.137931)");
        assert_eq!(color.ycbcr(), "YCbCr(0, 128, 128)");
        assert_eq!(color.lab(), "lab(0, 0, 0)");
        assert_eq!(color.name(), "#00000080");

        let color = Color::new(0.0, 128.0, 128.0, 1.0);
        assert_eq!(color.hex(), "#008080");
        assert_eq!(color.rgb(), "rgb(0, 128, 128)");
        assert_eq!(color.rgba(), "rgba(0, 128, 128, 1)");
        assert_eq!(color.hsl(), "hsl(180, 100%, 25%)");
        assert_eq!(color.hsv(), "hsv(180, 100%, 50%)");
        assert_eq!(color.hwb(), "hwb(180, 0%, 50%)");
        assert_eq!(color.xyz(), "xyz(0.496222, 0.553915, 0.596299)");
        assert_eq!(color.ycbcr(), "YCbCr(89.728, 149.5854, 64.0239)");
        assert_eq!(color.lab(), "lab(48.25, -28.85, -8.48)");
        assert_eq!(color.name(), "teal");
    }
}
