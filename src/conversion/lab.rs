use super::xyz::rgb2xyz;
use crate::helper::round;

#[allow(unused)]
pub fn rgb2lab(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = rgb2xyz(color);

    let l = 116.0 * y - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);

    let l = if l < 0.0 { 0.0 } else { l };

    (round(l, 4), round(a, 4), round(b, 4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2lab() {
        assert_eq!(rgb2lab((255.0, 255.0, 0.0)), (97.1393, -21.5537, 94.478));
        assert_eq!(rgb2lab((0.0, 255.0, 0.0)), (87.7347, -86.1827, 83.1793));
    }
}
