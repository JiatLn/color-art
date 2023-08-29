use crate::utils::{normalize_color, round};

/// Convert `YIQ` to `RGB`
///
/// reference: [From YIQ to RGB](https://en.wikipedia.org/wiki/YIQ#From_YIQ_to_RGB)
pub fn yiq2rgb(color: &[f64]) -> Vec<f64> {
    let y = color[0];
    let i = color[1];
    let q = color[2];
    let r = y + 0.956 * i + 0.619 * q;
    let g = y - 0.272 * i - 0.647 * q;
    let b = y - 1.106 * i + 1.703 * q;
    vec![
        round(r * 255.0, 0),
        round(g * 255.0, 0),
        round(b * 255.0, 0),
    ]
}

/// Convert `RGB` to `YIQ`
///
/// reference: [From RGB to YIQ](https://en.wikipedia.org/wiki/YIQ#From_RGB_to_YIQ)
pub fn rgb2yiq(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let i = 0.595716 * r - 0.274453 * g - 0.321263 * b;
    let q = 0.211456 * r - 0.522591 * g + 0.311135 * b;
    vec![y, i, q]
}

#[cfg(test)]
mod tests {

    use super::*;

    fn round5_vec(vec: Vec<f64>) -> Vec<f64> {
        vec.iter().map(|&v| round(v, 5)).collect::<Vec<_>>()
    }

    #[test]
    fn test_yiq2rgb() {
        let color = yiq2rgb(&[1.0, 0.0, 0.0]);
        assert_eq!(color, vec![255.0, 255.0, 255.0]);

        let color = yiq2rgb(&[0.42337, -0.07301, 0.17583]);
        assert_eq!(color, vec![118.0, 84.0, 205.0]);
    }

    #[test]
    fn test_rgb2yiq() {
        let color = rgb2yiq(&[255.0, 255.0, 255.0]);
        assert_eq!(round5_vec(color), vec![1.0, 0.0, 0.0]);

        let color = rgb2yiq(&[118.0, 84.0, 205.0]);
        assert_eq!(round5_vec(color), vec![0.42337, -0.07301, 0.17583]);
    }
}
