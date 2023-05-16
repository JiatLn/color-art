use crate::utils::*;

pub(crate) fn rgb2hsi(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let theta = (0.5 * (r - g + (r - b))) / ((r - g).powi(2) + (r - b) * (g - b)).sqrt();

    let theta = if theta.is_nan() { 0.0 } else { theta.acos().to_degrees() };

    let h = if b <= g { theta } else { 360.0 - theta };

    let min = r.min(g).min(b);

    let i = (r + g + b) / 3.0;

    let s = if i == 0.0 { 0.0 } else { 1.0 - min / i };

    (round(h, 4), round(s, 4), round(i, 4))
}

pub(crate) fn hsi2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (h, s, i) = color;

    let h = h % 360.0;
    let h = if h < 0.0 { 360.0 + h } else { h };

    let (r, g, b) = match h {
        h if h >= 0.0 && h < 120.0 => {
            let h = h.to_radians();

            let b = i * (1.0 - s);
            let r = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let g = 3.0 * i - (r + b);

            (r, g, b)
        }
        h if h >= 120.0 && h < 240.0 => {
            let h = (h - 120.0).to_radians();

            let r = i * (1.0 - s);
            let g = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let b = 3.0 * i - (r + g);

            (r, g, b)
        }
        h if h >= 240.0 && h < 360.0 => {
            let h = (h - 240.0).to_radians();

            let g = i * (1.0 - s);
            let b = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let r = 3.0 * i - (g + b);
            dbg!(r, g, b);
            (r, g, b)
        }
        _ => panic!("Hue must be between 0 and 360"),
    };

    (round(r * 255.0, 0), round(g * 255.0, 0), round(b * 255.0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hsi() {
        let color = (0.0, 0.0, 0.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (0.0, 0.0, 0.0));

        let color = (255.0, 255.0, 255.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (0.0, 0.0, 1.0));

        let color = (255.0, 0.0, 0.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (0.0, 1.0, 0.3333));

        let color = (0.0, 255.0, 0.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (120.0, 1.0, 0.3333));

        let color = (0.0, 0.0, 255.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (240.0, 1.0, 0.3333));

        let color = (255.0, 255.0, 0.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (60.0, 1.0, 0.6667));

        let color = (0.0, 255.0, 255.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (180.0, 1.0, 0.6667));

        let color = (255.0, 0.0, 255.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (300.0, 1.0, 0.6667));

        let color = (255.0, 128.0, 128.0);
        let hsi = rgb2hsi(color);
        assert_eq!(hsi, (0.0, 0.2485, 0.668));
    }

    #[test]
    fn test_hsi2rgb() {
        let color = (0.0, 0.0, 0.0);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (0.0, 0.0, 0.0));

        let color = (0.0, 0.0, 1.0);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (255.0, 255.0, 255.0));

        let color = (0.0, 1.0, 0.3333);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (255.0, 0.0, 0.0));

        let color = (120.0, 1.0, 0.3333);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (0.0, 255.0, 0.0));

        let color = (240.0, 1.0, 0.3333);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (0.0, 0.0, 255.0));

        let color = (60.0, 1.0, 0.6667);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (255.0, 255.0, 0.0));

        let color = (180.0, 1.0, 0.6667);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (0.0, 255.0, 255.0));

        let color = (300.0, 1.0, 0.6667);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (255.0, 0.0, 255.0));

        let color = (0.0, 0.2485, 0.668);
        let rgb = hsi2rgb(color);
        assert_eq!(rgb, (255.0, 128.0, 128.0));
    }
}
