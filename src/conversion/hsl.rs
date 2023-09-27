use crate::utils::*;

/// [HSL to RGB color conversion](https://www.rapidtables.com/convert/color/hsl-to-rgb.html)
pub fn hsl2rgb(color: &[f64]) -> Vec<f64> {
    let h = color[0];
    let s = color[1];
    let l = color[2];

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let rgb = match h {
        h if (0.0..60.0).contains(&h) => vec![c, x, 0.0],
        h if (60.0..120.0).contains(&h) => vec![x, c, 0.0],
        h if (120.0..180.0).contains(&h) => vec![0.0, c, x],
        h if (180.0..240.0).contains(&h) => vec![0.0, x, c],
        h if (240.0..300.0).contains(&h) => vec![x, 0.0, c],
        h if (300.0..360.0).contains(&h) => vec![c, 0.0, x],
        _ => panic!(),
    };

    rgb.iter().map(|x| (x + m) * 255.0).collect()
}

/// [RGB to HSL color conversion](https://www.rapidtables.com/convert/color/rgb-to-hsl.html)
pub fn rgb2hsl(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let mut h = 0.0;
    let mut s = 0.0;
    let l = (max + min) / 2.0;

    let delta = max - min;

    if delta != 0.0 {
        h = match max {
            x if x == r => 60.0 * (((g - b) / delta) % 6.0),
            x if x == g => 60.0 * ((b - r) / delta + 2.0),
            x if x == b => 60.0 * ((r - g) / delta + 4.0),
            _ => panic!(),
        };

        if h < 0.0 {
            h += 360.0;
        }

        s = delta / (1.0 - (2.0 * l - 1.0).abs());
        s = s.max(0.0).min(1.0);
    }

    vec![h, s, l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hsl() {
        let rgb = [41.0, 121.0, 255.0];
        let hsl = rgb2hsl(&rgb);
        assert_eq!(hsl[1], 1.0);
    }
}
