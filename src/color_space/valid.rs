use crate::ColorSpace;
use anyhow::Result;

impl ColorSpace {
    /// Check if a vector of values is valid for a given color space.
    pub(crate) fn valid(&self, vec: &Vec<f64>) -> Result<()> {
        match self {
            ColorSpace::RGB => {
                if vec.len() != 3 {
                    anyhow::bail!("RGB color space requires 3 values")
                }
                if let [r, g, b] = vec[..] {
                    if r < 0.0 || r > 255.0 {
                        anyhow::bail!("Red must be between 0 and 255, got {}", r)
                    }
                    if g < 0.0 || g > 255.0 {
                        anyhow::bail!("Green must be between 0 and 255, got {}", g)
                    }
                    if b < 0.0 || b > 255.0 {
                        anyhow::bail!("Blue must be between 0 and 255, got {}", b)
                    }
                }
            }
            ColorSpace::RGBA => {
                if vec.len() != 4 {
                    anyhow::bail!("RGBA color space requires 4 values")
                }
                if let [r, g, b, a] = vec[..] {
                    if r < 0.0 || r > 255.0 {
                        anyhow::bail!("Red must be between 0 and 255, got {}", r)
                    }
                    if g < 0.0 || g > 255.0 {
                        anyhow::bail!("Green must be between 0 and 255, got {}", g)
                    }
                    if b < 0.0 || b > 255.0 {
                        anyhow::bail!("Blue must be between 0 and 255, got {}", b)
                    }
                    if a < 0.0 || a > 1.0 {
                        anyhow::bail!("Alpha must be between 0.0 and 1.0, got {}", a)
                    }
                }
            }
            ColorSpace::HSL => {
                if vec.len() != 3 {
                    anyhow::bail!("HSL color space requires 3 values")
                }
                if let [h, s, l] = vec[..] {
                    if h < 0.0 || h > 360.0 {
                        anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                    }
                    if s < 0.0 || s > 1.0 {
                        anyhow::bail!("Saturation must be between 0.0 and 1.0, got {}", s)
                    }
                    if l < 0.0 || l > 1.0 {
                        anyhow::bail!("Lightness must be between 0.0 and 1.0, got {}", l)
                    }
                }
            }
            ColorSpace::HSLA => {
                if vec.len() != 4 {
                    anyhow::bail!("HSLA color space requires 4 values")
                }
                if let [h, s, l, a] = vec[..] {
                    if h < 0.0 || h > 360.0 {
                        anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                    }
                    if s < 0.0 || s > 1.0 {
                        anyhow::bail!("Saturation must be between 0.0 and 1.0, got {}", s)
                    }
                    if l < 0.0 || l > 1.0 {
                        anyhow::bail!("Lightness must be between 0.0 and 1.0, got {}", l)
                    }
                    if a < 0.0 || a > 1.0 {
                        anyhow::bail!("Alpha must be between 0.0 and 1.0, got {}", a)
                    }
                }
            }
            ColorSpace::HSV => {
                if vec.len() != 3 {
                    anyhow::bail!("HSV color space requires 3 values")
                }
                if let [h, s, v] = vec[..] {
                    if h < 0.0 || h > 360.0 {
                        anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                    }
                    if s < 0.0 || s > 1.0 {
                        anyhow::bail!("Saturation must be between 0.0 and 1.0, got {}", s)
                    }
                    if v < 0.0 || v > 1.0 {
                        anyhow::bail!("Value must be between 0.0 and 1.0, got {}", v)
                    }
                }
            }
            ColorSpace::HEX => todo!(
                "HEX color space not implemented yet, please use `ColorSpace::valid_hex` instead"
            ),
            ColorSpace::HWB => {
                if vec.len() != 3 {
                    anyhow::bail!("HWB color space requires 3 values")
                }
                if let [h, w, b] = vec[..] {
                    if h < 0.0 || h > 360.0 {
                        anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                    }
                    if w < 0.0 || w > 1.0 {
                        anyhow::bail!("Whiteness must be between 0.0 and 1.0, got {}", w)
                    }
                    if b < 0.0 || b > 1.0 {
                        anyhow::bail!("Blackness must be between 0.0 and 1.0, got {}", b)
                    }
                }
            }
            ColorSpace::CMYK => {
                if vec.len() != 4 {
                    anyhow::bail!("CMYK color space requires 4 values")
                }
                if let [c, m, y, k] = vec[..] {
                    if c < 0.0 || c > 1.0 {
                        anyhow::bail!("cyan color must be between 0.0 and 1.0, got {}", c)
                    }
                    if m < 0.0 || m > 1.0 {
                        anyhow::bail!("magenta color must be between 0.0 and 1.0, got {}", m)
                    }
                    if y < 0.0 || y > 1.0 {
                        anyhow::bail!("yellow color must be between 0.0 and 1.0, got {}", y)
                    }
                    if k < 0.0 || k > 1.0 {
                        anyhow::bail!("black color must be between 0.0 and 1.0, got {}", k)
                    }
                }
            }
            ColorSpace::XYZ => {
                if vec.len() != 3 {
                    anyhow::bail!("XYZ color space requires 3 values")
                }
                if let [x, y, z] = vec[..] {
                    if x < 0.0 || x > 0.950456 {
                        anyhow::bail!("X must be between 0.0 and 0.950456, got {}", x)
                    }
                    if y < 0.0 || y > 1.0 {
                        anyhow::bail!("Y must be between 0.0 and 1.0, got {}", y)
                    }
                    if z < 0.0 || z > 1.088754 {
                        anyhow::bail!("Z must be between 0.0 and 1.088754, got {}", z)
                    }
                }
            }
            ColorSpace::YUV => {
                if vec.len() != 3 {
                    anyhow::bail!("YUV color space requires 3 values")
                }
                if let [y, u, v] = vec[..] {
                    if y < 0.0 || y > 1.0 {
                        anyhow::bail!("Y must be between 0.0 and 1.0, got {}", y)
                    }
                    if u < -0.436 || u > 0.436 {
                        anyhow::bail!("U must be between -0.436 and 0.436, got {}", u)
                    }
                    if v < -0.615 || v > 0.615 {
                        anyhow::bail!("V must be between -0.615 and 0.615, got {}", v)
                    }
                }
            }
            ColorSpace::YCbCr => {
                if vec.len() != 3 {
                    anyhow::bail!("YCbCr color space requires 3 values")
                }
                if let [y, cb, cr] = vec[..] {
                    if y < 0.0 || y > 255.0 {
                        anyhow::bail!("Y must be between 0.0 and 255.0, got {}", y)
                    }
                    if cb < 0.0 || cb > 255.0 {
                        anyhow::bail!("Cb must be between 0.0 and 255.0, got {}", cb)
                    }
                    if cr < 0.0 || cr > 255.0 {
                        anyhow::bail!("Cr must be between 0.0 and 255.0, got {}", cr)
                    }
                }
            }
            ColorSpace::Lab => {
                if vec.len() != 3 {
                    anyhow::bail!("Lab color space requires 3 values")
                }
                if let [l, a, b] = vec[..] {
                    if l < 0.0 || l > 100.0 {
                        anyhow::bail!("L must be between 0.0 and 100.0, got {}", l)
                    }
                    if a < -128.0 || a > 127.0 {
                        anyhow::bail!("A must be between -128.0 and 127.0, got {}", a)
                    }
                    if b < -128.0 || b > 127.0 {
                        anyhow::bail!("B must be between -128.0 and 127.0, got {}", b)
                    }
                }
            }
            ColorSpace::Unknown => todo!("Unknown color space validation"),
        }
        Ok(())
    }
    /// Validate a hex color string
    pub(crate) fn valid_hex(hex: &str) -> Result<()> {
        if !hex.chars().skip(1).all(|c| c.is_ascii_hexdigit()) {
            anyhow::bail!("Hex color must be a valid hex string")
        }
        if hex.len() != 4 && hex.len() != 5 && hex.len() != 7 && hex.len() != 9 {
            anyhow::bail!("Hex color must be 3, 4, 6, or 8 characters long")
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
