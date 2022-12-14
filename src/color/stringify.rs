use crate::{
    conversion::{cmyk::rgb2cmyk, hex::rgb2hex, hsl::rgb2hsl, hsv::rgb2hsv, hwb::rgb2hwb},
    helper::round,
    Color,
};

/// Stringify a color to a string.
impl Color {
    /// `hex` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hex(), "#ffffff");
    /// ```
    pub fn hex(self) -> String {
        let (r, g, b, _) = self.rgba;
        rgb2hex((r, g, b))
    }
    /// `rgb` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.rgb(), "rgb(255, 255, 255)");
    /// ```
    pub fn rgb(self) -> String {
        let (r, g, b, _) = self.rgba;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgb({}, {}, {})", r, g, b)
    }
    /// `rgba` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 0.5);
    /// assert_eq!(color.rgba(), "rgba(255, 255, 255, 0.5)");
    /// ```
    pub fn rgba(self) -> String {
        let (r, g, b, a) = self.rgba;
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        format!("rgba({}, {}, {}, {})", r, g, b, a)
    }
    /// `hsl` string of the color
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hsl(), "hsl(0, 0%, 100%)");
    /// ```
    pub fn hsl(self) -> String {
        let (r, g, b, _) = self.rgba;
        let (h, s, l) = rgb2hsl((r, g, b));
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
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hsv(), "hsv(0, 0%, 100%)");
    /// ```
    pub fn hsv(self) -> String {
        let (r, g, b, _) = self.rgba;
        let (h, s, v) = rgb2hsv((r, g, b));
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
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.hwb(), "hwb(0, 100%, 0%)");
    /// ```
    pub fn hwb(self) -> String {
        let (r, g, b, _) = self.rgba;
        let (h, w, b) = rgb2hwb((r, g, b));
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
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.cmyk(), "cmyk(0%, 0%, 0%, 0%)");
    /// ```
    pub fn cmyk(self) -> String {
        let (r, g, b, _) = self.rgba;
        let (c, m, y, k) = rgb2cmyk((r, g, b));
        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            round(c * 100., 0),
            round(m * 100., 0),
            round(y * 100., 0),
            round(k * 100., 0),
        )
    }
    /// `name` of the color
    ///
    /// If the color is not in the [*w3cx11*](http://www.w3.org/TR/css3-color/#svg-color) color list, the hex string will be returned.
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::new(255.0, 255.0, 255.0, 1.0);
    /// assert_eq!(color.name(), "white");
    ///
    /// let color = Color::new(0.0, 42.0, 0.0, 1.0);
    /// assert_eq!(color.name(), "#002a00");
    /// ```
    pub fn name(self) -> String {
        let hex = self.hex();

        let result = crate::W3CX11
            .clone()
            .into_iter()
            .find(|(_k, v)| v.to_string() == hex);

        match result {
            Some((k, _v)) => String::from(k),
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

        let color = Color::new(0.0, 0.0, 0.0, 0.5);
        assert_eq!(color.hex(), "#000000");
        assert_eq!(color.rgb(), "rgb(0, 0, 0)");
        assert_eq!(color.rgba(), "rgba(0, 0, 0, 0.5)");
        assert_eq!(color.hsl(), "hsl(0, 0%, 0%)");
        assert_eq!(color.hsv(), "hsv(0, 0%, 0%)");
        assert_eq!(color.hwb(), "hwb(0, 0%, 100%)");

        let color = Color::new(0.0, 128.0, 128.0, 1.0);
        assert_eq!(color.hex(), "#008080");
        assert_eq!(color.rgb(), "rgb(0, 128, 128)");
        assert_eq!(color.rgba(), "rgba(0, 128, 128, 1)");
        assert_eq!(color.hsl(), "hsl(180, 100%, 25%)");
        assert_eq!(color.hsv(), "hsv(180, 100%, 50%)");
        assert_eq!(color.hwb(), "hwb(180, 0%, 50%)");
    }
}
