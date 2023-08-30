use crate::{
    conversion::{
        cmyk::rgb2cmyk,
        hex::{rgb2hex, rgba2hex},
        hsi::rgb2hsi,
        hsl::rgb2hsl,
        hsv::rgb2hsv,
        hwb::rgb2hwb,
        lab::rgb2lab,
        xyz::rgb2xyz,
        ycbcr::rgb2ycbcr,
        yiq::rgb2yiq,
        yuv::rgb2yuv,
    },
    data::name_of_hex,
    utils::{hex::simplify_hex, round},
    Color,
};

/// Stringify a color to a string.
impl Color {
    /// `hex` string of the color
    ///
    /// The hex string is simplified to a short hex string if possible.
    ///
    /// For example:
    /// - `#ff00ff` -> `#f0f`
    /// - `#ffffff88` -> `#fff8`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255, 0, 255, 1.0);
    /// assert_eq!(color.hex(), "#f0f");
    ///
    /// let color = Color::new(255, 255, 255, 0.5);
    /// assert_eq!(color.hex(), "#ffffff80");
    /// ```
    pub fn hex(self) -> String {
        simplify_hex(self.hex_full())
    }
    /// `hex` string of the color with the full length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255, 0, 255, 1.0);
    /// assert_eq!(color.hex_full(), "#ff00ff");
    /// ```
    pub fn hex_full(self) -> String {
        if self.alpha == 1.0 {
            rgb2hex(self.rgb)
        } else {
            let [r, g, b] = self.rgb;
            rgba2hex([r, g, b, self.alpha])
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
        let [r, g, b] = self.rgb;
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
        let [r, g, b] = self.rgb;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgba({}, {}, {}, {})", r, g, b, self.alpha())
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
        let hsl = rgb2hsl(&self.rgb);
        let h = round(hsl[0], 0);
        let s = round(hsl[1] * 100.0, 0);
        let l = round(hsl[2] * 100.0, 0);
        format!("hsl({}, {}%, {}%)", h, s, l)
    }
    /// `hsla` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255, 255, 255, 0.3);
    /// assert_eq!(color.hsla(), "hsla(0, 0%, 100%, 0.3)");
    /// ```
    pub fn hsla(self) -> String {
        let hsl = rgb2hsl(&self.rgb);
        let h = round(hsl[0], 0);
        let s = round(hsl[1] * 100.0, 0);
        let l = round(hsl[2] * 100.0, 0);
        format!("hsla({}, {}%, {}%, {})", h, s, l, self.alpha())
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
        let hsv = rgb2hsv(&self.rgb);
        let h = round(hsv[0], 0);
        let s = round(hsv[1] * 100.0, 0);
        let v = round(hsv[2] * 100.0, 0);
        format!("hsv({}, {}%, {}%)", h, s, v)
    }
    /// `hsi` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255, 255, 255, 1.0);
    /// assert_eq!(color.hsi(), "hsi(0, 0%, 100%)");
    /// ```
    pub fn hsi(self) -> String {
        let hsi = rgb2hsi(&self.rgb);
        let h = round(hsi[0], 0);
        let s = round(hsi[1] * 100.0, 2);
        let i = round(hsi[2] * 100.0, 2);
        format!("hsi({}, {}%, {}%)", h, s, i)
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
        let hwb = rgb2hwb(&self.rgb);
        let h = round(hwb[0], 0);
        let w = round(hwb[1] * 100.0, 0);
        let b = round(hwb[2] * 100.0, 0);
        format!("hwb({}, {}%, {}%)", h, w, b)
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
        let cmyk = rgb2cmyk(&self.rgb)
            .iter()
            .map(|&v| round(v * 100.0, 0))
            .collect::<Vec<_>>();
        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            cmyk[0], cmyk[1], cmyk[2], cmyk[3]
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
    /// assert_eq!(color.xyz(), "xyz(0.412391, 0.212639, 0.019331)");
    /// ```
    pub fn xyz(self) -> String {
        let xyz = rgb2xyz(&self.rgb)
            .iter()
            .map(|&v| round(v, 6))
            .collect::<Vec<_>>();
        format!("xyz({}, {}, {})", xyz[0], xyz[1], xyz[2])
    }
    /// `yiq` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 0.0, 0.0, 1.0);
    /// assert_eq!(color.yiq(), "yiq(0.299, 0.59572, 0.21146)");
    /// ```
    pub fn yiq(self) -> String {
        let yiq = rgb2yiq(&self.rgb)
            .iter()
            .map(|&v| round(v, 5))
            .collect::<Vec<_>>();
        format!("yiq({}, {}, {})", yiq[0], yiq[1], yiq[2])
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
        let yuv = rgb2yuv(&self.rgb)
            .iter()
            .map(|&v| round(v, 4))
            .collect::<Vec<_>>();
        format!("yuv({}, {}, {})", yuv[0], yuv[1], yuv[2])
    }
    /// `lab` string of the color
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 0.0, 1.0);
    /// assert_eq!(color.lab(), "lab(97.61, -15.75, 93.39)");
    /// ```
    pub fn lab(self) -> String {
        let lab = rgb2lab(&self.rgb)
            .iter()
            .map(|&v| round(v, 2))
            .collect::<Vec<_>>();
        format!("lab({}, {}, {})", lab[0], lab[1], lab[2])
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
        let ycbcr = rgb2ycbcr(&self.rgb)
            .iter()
            .map(|&v| round(v, 4))
            .collect::<Vec<_>>();
        format!("YCbCr({}, {}, {})", ycbcr[0], ycbcr[1], ycbcr[2])
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
    /// use color_art::{Color, color};
    ///
    /// let color = color!(#ffffff);
    /// assert_eq!(color.name(), "white");
    ///
    /// let color = color!(#f8df72);
    /// assert_eq!(color.name(), "茉莉黄");
    ///
    /// let color = Color::new(42, 42, 42, 1.0);
    /// assert_eq!(color.name(), "#2a2a2a");
    /// ```
    pub fn name(self) -> String {
        if self.alpha == 1.0 {
            let hex = rgb2hex(self.rgb);
            match name_of_hex(&hex) {
                Some(name) => name.to_string(),
                None => hex,
            }
        } else {
            self.hex()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify_color() {
        let color = Color::new(255.0, 255.0, 255.0, 1.0);
        assert_eq!(color.hex(), "#fff");
        assert_eq!(color.hex_full(), "#ffffff");
        assert_eq!(color.rgb(), "rgb(255, 255, 255)");
        assert_eq!(color.rgba(), "rgba(255, 255, 255, 1)");
        assert_eq!(color.hsl(), "hsl(0, 0%, 100%)");
        assert_eq!(color.hsla(), "hsla(0, 0%, 100%, 1)");
        assert_eq!(color.hsv(), "hsv(0, 0%, 100%)");
        assert_eq!(color.hsi(), "hsi(0, 0%, 100%)");
        assert_eq!(color.hwb(), "hwb(0, 100%, 0%)");
        assert_eq!(color.xyz(), "xyz(0.950456, 1, 1.089058)");
        assert_eq!(color.ycbcr(), "YCbCr(255, 128, 128)");
        assert_eq!(color.lab(), "lab(100, 0, 0)");
        assert_eq!(color.name(), "white");

        let color = Color::new(0.0, 0.0, 0.0, 0.2);
        assert_eq!(color.hex(), "#0003");
        assert_eq!(color.hex_full(), "#00000033");
        assert_eq!(color.rgb(), "rgb(0, 0, 0)");
        assert_eq!(color.rgba(), "rgba(0, 0, 0, 0.2)");
        assert_eq!(color.hsl(), "hsl(0, 0%, 0%)");
        assert_eq!(color.hsla(), "hsla(0, 0%, 0%, 0.2)");
        assert_eq!(color.hsv(), "hsv(0, 0%, 0%)");
        assert_eq!(color.hsi(), "hsi(0, 0%, 0%)");
        assert_eq!(color.hwb(), "hwb(0, 0%, 100%)");
        assert_eq!(color.xyz(), "xyz(0, 0, 0)");
        assert_eq!(color.ycbcr(), "YCbCr(0, 128, 128)");
        assert_eq!(color.lab(), "lab(0, 0, 0)");
        assert_eq!(color.name(), "#0003");

        let color = Color::new(0.0, 128.0, 128.0, 1.0);
        assert_eq!(color.hex(), "#008080");
        assert_eq!(color.hex_full(), "#008080");
        assert_eq!(color.rgb(), "rgb(0, 128, 128)");
        assert_eq!(color.rgba(), "rgba(0, 128, 128, 1)");
        assert_eq!(color.hsl(), "hsl(180, 100%, 25%)");
        assert_eq!(color.hsla(), "hsla(180, 100%, 25%, 1)");
        assert_eq!(color.hsv(), "hsv(180, 100%, 50%)");
        assert_eq!(color.hsi(), "hsi(180, 100%, 33.46%)");
        assert_eq!(color.hwb(), "hwb(180, 0%, 50%)");
        assert_eq!(color.xyz(), "xyz(0.116147, 0.16996, 0.230912)");
        assert_eq!(color.ycbcr(), "YCbCr(89.728, 149.5854, 64.0239)");
        assert_eq!(color.lab(), "lab(47.99, -30.39, -8.98)");
        assert_eq!(color.name(), "teal");

        let color = Color::new(161, 110, 87, 1.0);
        assert_eq!(color.hex(), "#a16e57");
        assert_eq!(color.hex_full(), "#a16e57");
        assert_eq!(color.rgb(), "rgb(161, 110, 87)");
        assert_eq!(color.rgba(), "rgba(161, 110, 87, 1)");
        assert_eq!(color.hsl(), "hsl(19, 30%, 49%)");
        assert_eq!(color.hsla(), "hsla(19, 30%, 49%, 1)");
        assert_eq!(color.hsv(), "hsv(19, 46%, 63%)");
        assert_eq!(color.hsi(), "hsi(18, 27.09%, 46.8%)");
        assert_eq!(color.hwb(), "hwb(19, 34%, 37%)");
        assert_eq!(color.xyz(), "xyz(0.219934, 0.194179, 0.116068)");
        assert_eq!(color.ycbcr(), "YCbCr(122.627, 107.9064, 155.3599)");
        assert_eq!(color.lab(), "lab(51.48, 18.82, 21.44)");
        assert_eq!(color.name(), "#a16e57");
    }
}
