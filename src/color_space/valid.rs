use crate::ColorSpace;
use anyhow::Result;

impl ColorSpace {
    /// Check if a vector of values is valid for a given color space.
    pub fn valid(&self, vec: &Vec<f64>) -> Result<()> {
        match self {
            ColorSpace::RGB => {
                if vec.len() != 3 {
                    anyhow::bail!("RGB color space requires 3 values")
                }
                let r = vec[0];
                let g = vec[1];
                let b = vec[2];
                if r < 0.0 || r > 255.0 {
                    anyhow::bail!("Red must be between 0 and 255, got {}", r)
                }
                if g < 0.0 || g > 255.0 {
                    anyhow::bail!("Green must be between 0 and 255, got {}", g)
                }
                if b < 0.0 || b > 255.0 {
                    anyhow::bail!("Blue must be between 0 and 255, got {}", b)
                }
                Ok(())
            }
            ColorSpace::RGBA => {
                if vec.len() != 4 {
                    anyhow::bail!("RGBA color space requires 4 values")
                }
                let r = vec[0];
                let g = vec[1];
                let b = vec[2];
                let a = vec[3];
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
                Ok(())
            }
            ColorSpace::HSL => {
                if vec.len() != 3 {
                    anyhow::bail!("HSL color space requires 3 values")
                }
                let h = vec[0];
                let s = vec[1];
                let l = vec[2];
                if h < 0.0 || h > 360.0 {
                    anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                }
                if s < 0.0 || s > 1.0 {
                    anyhow::bail!("Saturation must be between 0.0 and 1.0, got {}", s)
                }
                if l < 0.0 || l > 1.0 {
                    anyhow::bail!("Lightness must be between 0.0 and 1.0, got {}", l)
                }
                Ok(())
            }
            ColorSpace::HSV => {
                if vec.len() != 3 {
                    anyhow::bail!("HSV color space requires 3 values")
                }
                let h = vec[0];
                let s = vec[1];
                let v = vec[2];
                if h < 0.0 || h > 360.0 {
                    anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                }
                if s < 0.0 || s > 1.0 {
                    anyhow::bail!("Saturation must be between 0.0 and 1.0, got {}", s)
                }
                if v < 0.0 || v > 1.0 {
                    anyhow::bail!("Value must be between 0.0 and 1.0, got {}", v)
                }
                Ok(())
            }
            ColorSpace::HEX => todo!(),
            ColorSpace::HWB => {
                if vec.len() != 3 {
                    anyhow::bail!("HWB color space requires 3 values")
                }
                let h = vec[0];
                let w = vec[1];
                let b = vec[2];
                if h < 0.0 || h > 360.0 {
                    anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                }
                if w < 0.0 || w > 1.0 {
                    anyhow::bail!("Whiteness must be between 0.0 and 1.0, got {}", w)
                }
                if b < 0.0 || b > 1.0 {
                    anyhow::bail!("Blackness must be between 0.0 and 1.0, got {}", b)
                }
                Ok(())
            }
            ColorSpace::CMYK => {
                if vec.len() != 4 {
                    anyhow::bail!("CMYK color space requires 4 values")
                }
                let c = vec[0];
                let m = vec[1];
                let y = vec[2];
                let k = vec[3];
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
                Ok(())
            }
            ColorSpace::Unknown => todo!(),
        }
    }
}
