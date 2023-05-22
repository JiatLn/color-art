use crate::utils::*;

pub(crate) fn rgb2hsi(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let min = r.min(g).min(b);

    let theta = (0.5 * (r - g + (r - b))) / ((r - g).powi(2) + (r - b) * (g - b)).sqrt();
    let theta = if theta.is_nan() { 0.0 } else { theta.acos().to_degrees() };

    let h = if b <= g { theta } else { 360.0 - theta };
    let i = (r + g + b) / 3.0;
    let s = if i == 0.0 { 0.0 } else { 1.0 - min / i };

    vec![h, s, i]
}

pub(crate) fn hsi2rgb(color: &[f64]) -> Vec<f64> {
    let h = color[0];
    let s = color[1];
    let i = color[2];

    let h = h % 360.0;
    let h = if h < 0.0 { 360.0 + h } else { h };

    let rgb = match h {
        h if h >= 0.0 && h < 120.0 => {
            let h = h.to_radians();

            let b = i * (1.0 - s);
            let r = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let g = 3.0 * i - (r + b);

            vec![r, g, b]
        }
        h if h >= 120.0 && h < 240.0 => {
            let h = (h - 120.0).to_radians();

            let r = i * (1.0 - s);
            let g = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let b = 3.0 * i - (r + g);

            vec![r, g, b]
        }
        h if h >= 240.0 && h < 360.0 => {
            let h = (h - 240.0).to_radians();

            let g = i * (1.0 - s);
            let b = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let r = 3.0 * i - (g + b);

            vec![r, g, b]
        }
        _ => panic!("Hue must be between 0 and 360"),
    };

    rgb.iter()
        .map(|&x| round(x * 255.0, 0))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_rgb2hsi() {
        let color = vec![0.0, 0.0, 0.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![0.0, 0.0, 0.0]);

        let color = vec![255.0, 255.0, 255.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![0.0, 0.0, 1.0]);

        let color = vec![255.0, 0.0, 0.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![0.0, 1.0, 0.3333333333333333]);

        let color = vec![0.0, 255.0, 0.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![120.00000000000001, 1.0, 0.3333333333333333]);

        let color = vec![0.0, 0.0, 255.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![240.0, 1.0, 0.3333333333333333]);

        let color = vec![255.0, 255.0, 0.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![60.00000000000001, 1.0, 0.6666666666666666]);

        let color = vec![0.0, 255.0, 255.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![180.0, 1.0, 0.6666666666666666]);

        let color = vec![255.0, 0.0, 255.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![300.0, 1.0, 0.6666666666666666]);

        let color = vec![255.0, 128.0, 128.0];
        let hsi = rgb2hsi(&color);
        assert_eq!(hsi, vec![0.0, 0.24853228962817997, 0.6679738562091503]);
    }

    #[test]
    fn test_hsi2rgb() {
        let color = vec![0.0, 0.0, 0.0];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![0.0, 0.0, 0.0]);

        let color = vec![0.0, 0.0, 1.0];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![255.0, 255.0, 255.0]);

        let color = vec![0.0, 1.0, 0.3333];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![255.0, 0.0, 0.0]);

        let color = vec![120.0, 1.0, 0.3333];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![0.0, 255.0, 0.0]);

        let color = vec![240.0, 1.0, 0.3333];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![0.0, 0.0, 255.0]);

        let color = vec![60.0, 1.0, 0.6667];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![255.0, 255.0, 0.0]);

        let color = vec![180.0, 1.0, 0.6667];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![0.0, 255.0, 255.0]);

        let color = vec![300.0, 1.0, 0.6667];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![255.0, 0.0, 255.0]);

        let color = vec![0.0, 0.2485, 0.668];
        let rgb = hsi2rgb(&color);
        assert_eq!(rgb, vec![255.0, 128.0, 128.0]);
    }
}
