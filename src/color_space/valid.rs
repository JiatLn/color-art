use crate::{ColorSpace, Error};

impl ColorSpace {
    /// Check if a vector of values is valid for a given color space.
    pub(crate) fn valid(&self, vec: &[f64]) -> Result<(), Error> {
        let result = match self {
            ColorSpace::RGB => valid_rgb(vec),
            ColorSpace::RGBA => valid_rgba(vec),
            ColorSpace::HSI => valid_hsi(vec),
            ColorSpace::HSL => valid_hsl(vec),
            ColorSpace::HSLA => valid_hsla(vec),
            ColorSpace::HSV => valid_hsv(vec),
            ColorSpace::HWB => valid_hwb(vec),
            ColorSpace::CMYK => valid_cmyk(vec),
            ColorSpace::XYZ => valid_xyz(vec),
            ColorSpace::YIQ => valid_yiq(vec),
            ColorSpace::YUV => valid_yuv(vec),
            ColorSpace::YCbCr => valid_ycbcr(vec),
            ColorSpace::Lab => valid_lab(vec),
            ColorSpace::HEX | ColorSpace::HEXA => Some(
                "HEX color space not implemented yet, please use `ColorSpace::valid_hex` instead"
                    .to_string(),
            ),
            ColorSpace::Unknown => todo!("Unknown color space validation"),
        };
        match result {
            Some(info) => Err(Error::ColorParserError(info)),
            None => Ok(()),
        }
    }
    /// Validate a hex color string
    pub(crate) fn valid_hex(hex: &str) -> Result<(), Error> {
        if !hex.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            Err(Error::ColorParserError(
                "Hex color string must be a valid hex string".to_string(),
            ))
        } else if hex.len() != 4 && hex.len() != 5 && hex.len() != 7 && hex.len() != 9 {
            Err(Error::ColorParserError(
                "Hex color must be 3, 4, 6, or 8 characters long".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

fn valid_yiq(vec: &[f64]) -> Option<String> {
    if let [y, i, q] = vec[..] {
        if !(0.0..=1.0).contains(&y) {
            Some(format!("Y must be between 0.0 and 1.0, got {}", y))
        } else if !(-0.5957..=0.5957).contains(&i) {
            Some(format!("I must be between -0.5957 and 0.5957, got {}", i))
        } else if !(-0.5226..=0.5226).contains(&q) {
            Some(format!("Q must be between -0.5226 and 0.5226, got {}", q))
        } else {
            None
        }
    } else {
        Some("YIQ color space requires 3 values".to_string())
    }
}

fn valid_xyz(vec: &[f64]) -> Option<String> {
    if let [x, y, z] = vec[..] {
        if !(0.0..=0.950456).contains(&x) {
            Some(format!("X must be between 0.0 and 0.950456, got {}", x))
        } else if !(0.0..=1.0).contains(&y) {
            Some(format!("Y must be between 0.0 and 1.0, got {}", y))
        } else if !(0.0..=1.088754).contains(&z) {
            Some(format!("Z must be between 0.0 and 1.088754, got {}", z))
        } else {
            None
        }
    } else {
        Some("XYZ color space requires 3 values".to_string())
    }
}

fn valid_cmyk(vec: &[f64]) -> Option<String> {
    if let [c, m, y, k] = vec[..] {
        if !(0.0..=1.0).contains(&c) {
            Some(format!("cyan color must be between 0.0 and 1.0, got {}", c))
        } else if !(0.0..=1.0).contains(&m) {
            Some(format!(
                "magenta color must be between 0.0 and 1.0, got {}",
                m
            ))
        } else if !(0.0..=1.0).contains(&y) {
            Some(format!(
                "yellow color must be between 0.0 and 1.0, got {}",
                y
            ))
        } else if !(0.0..=1.0).contains(&k) {
            Some(format!(
                "black color must be between 0.0 and 1.0, got {}",
                k
            ))
        } else {
            None
        }
    } else {
        Some("CMYK color space requires 4 values".to_string())
    }
}

fn valid_hwb(vec: &[f64]) -> Option<String> {
    if let [h, w, b] = vec[..] {
        if !(0.0..=360.0).contains(&h) {
            Some(format!("Hue must be between 0.0 and 360.0, got {}", h))
        } else if !(0.0..=1.0).contains(&w) {
            Some(format!("Whiteness must be between 0.0 and 1.0, got {}", w))
        } else if !(0.0..=1.0).contains(&b) {
            Some(format!("Blackness must be between 0.0 and 1.0, got {}", b))
        } else {
            None
        }
    } else {
        Some("HWB color space requires 3 values".to_string())
    }
}

fn valid_rgb(vec: &[f64]) -> Option<String> {
    if let [r, g, b] = vec[..] {
        if !(0.0..=255.0).contains(&r) {
            Some(format!("Red must be between 0 and 255, got {}", r))
        } else if !(0.0..=255.0).contains(&g) {
            Some(format!("Green must be between 0 and 255, got {}", g))
        } else if !(0.0..=255.0).contains(&b) {
            Some(format!("Blue must be between 0 and 255, got {}", b))
        } else {
            None
        }
    } else {
        Some("RGB color space requires 3 values".to_string())
    }
}

fn valid_rgba(vec: &[f64]) -> Option<String> {
    if let [r, g, b, a] = vec[..] {
        if !(0.0..=255.0).contains(&r) {
            Some(format!("Red must be between 0 and 255, got {}", r))
        } else if !(0.0..=255.0).contains(&g) {
            Some(format!("Green must be between 0 and 255, got {}", g))
        } else if !(0.0..=255.0).contains(&b) {
            Some(format!("Blue must be between 0 and 255, got {}", b))
        } else if !(0.0..=1.0).contains(&a) {
            Some(format!("Alpha must be between 0.0 and 1.0, got {}", a))
        } else {
            None
        }
    } else {
        Some("RGBA color space requires 4 values".to_string())
    }
}

fn valid_hsi(vec: &[f64]) -> Option<String> {
    if let [h, s, i] = vec[..] {
        if !(0.0..=360.0).contains(&h) {
            Some(format!("Hue must be between 0.0 and 360.0, got {}", h))
        } else if !(0.0..=1.0).contains(&s) {
            Some(format!("Saturation must be between 0.0 and 1.0, got {}", s))
        } else if !(0.0..=1.0).contains(&i) {
            Some(format!("Intensity must be between 0.0 and 1.0, got {}", i))
        } else {
            None
        }
    } else {
        Some("HSI color space requires 3 values".to_string())
    }
}

fn valid_hsl(vec: &[f64]) -> Option<String> {
    if let [h, s, l] = vec[..] {
        if !(0.0..=360.0).contains(&h) {
            Some(format!("Hue must be between 0.0 and 360.0, got {}", h))
        } else if !(0.0..=1.0).contains(&s) {
            Some(format!("Saturation must be between 0.0 and 1.0, got {}", s))
        } else if !(0.0..=1.0).contains(&l) {
            Some(format!("Lightness must be between 0.0 and 1.0, got {}", l))
        } else {
            None
        }
    } else {
        Some("HSL color space requires 3 values".to_string())
    }
}

fn valid_hsla(vec: &[f64]) -> Option<String> {
    if let [h, s, l, a] = vec[..] {
        if !(0.0..=360.0).contains(&h) {
            Some(format!("Hue must be between 0.0 and 360.0, got {}", h))
        } else if !(0.0..=1.0).contains(&s) {
            Some(format!("Saturation must be between 0.0 and 1.0, got {}", s))
        } else if !(0.0..=1.0).contains(&l) {
            Some(format!("Lightness must be between 0.0 and 1.0, got {}", l))
        } else if !(0.0..=1.0).contains(&a) {
            Some(format!("Alpha must be between 0.0 and 1.0, got {}", a))
        } else {
            None
        }
    } else {
        Some("HSLA color space requires 4 values".to_string())
    }
}

fn valid_hsv(vec: &[f64]) -> Option<String> {
    if let [h, s, v] = vec[..] {
        if !(0.0..=360.0).contains(&h) {
            Some(format!("Hue must be between 0.0 and 360.0, got {}", h))
        } else if !(0.0..=1.0).contains(&s) {
            Some(format!("Saturation must be between 0.0 and 1.0, got {}", s))
        } else if !(0.0..=1.0).contains(&v) {
            Some(format!("Value must be between 0.0 and 1.0, got {}", v))
        } else {
            None
        }
    } else {
        Some("HSV color space requires 3 values".to_string())
    }
}

fn valid_lab(vec: &[f64]) -> Option<String> {
    if let [l, a, b] = vec[..] {
        if !(0.0..=100.0).contains(&l) {
            Some(format!("L must be between 0.0 and 100.0, got {}", l))
        } else if !(-128.0..=127.0).contains(&a) {
            Some(format!("A must be between -128.0 and 127.0, got {}", a))
        } else if !(-128.0..=127.0).contains(&b) {
            Some(format!("B must be between -128.0 and 127.0, got {}", b))
        } else {
            None
        }
    } else {
        Some("Lab color space requires 3 values".to_string())
    }
}

fn valid_yuv(vec: &[f64]) -> Option<String> {
    if let [y, u, v] = vec[..] {
        if !(0.0..=1.0).contains(&y) {
            Some(format!("Y must be between 0.0 and 1.0, got {}", y))
        } else if !(-0.436..=0.436).contains(&u) {
            Some(format!("U must be between -0.436 and 0.436, got {}", u))
        } else if !(-0.615..=0.615).contains(&v) {
            Some(format!("V must be between -0.615 and 0.615, got {}", v))
        } else {
            None
        }
    } else {
        Some("YUV color space requires 3 values".to_string())
    }
}

fn valid_ycbcr(vec: &[f64]) -> Option<String> {
    if let [y, cb, cr] = vec[..] {
        if !(0.0..=255.0).contains(&y) {
            Some(format!("Y must be between 0.0 and 255.0, got {}", y))
        } else if !(0.0..=255.0).contains(&cb) {
            Some(format!("Cb must be between 0.0 and 255.0, got {}", cb))
        } else if !(0.0..=255.0).contains(&cr) {
            Some(format!("Cr must be between 0.0 and 255.0, got {}", cr))
        } else {
            None
        }
    } else {
        Some("YCbCr color space requires 3 values".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yuv() {
        let yuv = [0.0, 0.0, 0.0, 0.0];
        let result = valid_yuv(&yuv);
        assert_eq!(
            result,
            Some("YUV color space requires 3 values".to_string())
        );

        let yuv = [0.0, 10.0, 0.0];
        let result = valid_yuv(&yuv);
        assert_eq!(
            result,
            Some("U must be between -0.436 and 0.436, got 10".to_string())
        );

        let yuv = [0.0, 0.0, 0.0];
        let result = valid_yuv(&yuv);
        assert_eq!(result, None);
    }

    #[test]
    fn test_valid_hex() {
        let hex = "#f39";
        assert!(ColorSpace::valid_hex(hex).is_ok());

        let hex = "#f39e";
        assert!(ColorSpace::valid_hex(hex).is_ok());

        let hex = "#ff3399";
        assert!(ColorSpace::valid_hex(hex).is_ok());

        let hex = "#ff3399ff";
        assert!(ColorSpace::valid_hex(hex).is_ok());

        let hex = "#ff333";
        assert!(ColorSpace::valid_hex(hex).is_err());

        let hex = "#zzz";
        assert!(ColorSpace::valid_hex(hex).is_err());

        let hex = "#ff3399f";
        assert!(ColorSpace::valid_hex(hex).is_err());

        let hex = "not a hex color";
        assert!(ColorSpace::valid_hex(hex).is_err());
    }
}
