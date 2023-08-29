use crate::{Color, ColorSpace};

/// Computes [color difference](https://en.wikipedia.org/wiki/Color_difference#CIEDE2000) as developed by the International Commission on Illumination (CIE) in 2000.
///
/// The implementation is based on the formula from [Bruce Lindbloom](http://www.brucelindbloom.com/index.html?Eqn_DeltaE_CIE2000.html).
///
/// Resulting values range from 0 (no difference) to 100 (maximum difference), and are a metric for how the human eye percieves color difference.
///
/// # Examples
///
/// ```
/// use color_art::{delta_e, color};
///
/// let color1 = color!(#fefe0e);
/// let color2 = color!(#fff);
///
/// fn is_equal(a: f64, b: f64) -> bool {
///   (a - b).abs() < 0.000001
/// }
///
/// let d = delta_e(&color1, &color2);
/// assert!(is_equal(d, 30.165629067733235));
/// ```
pub fn delta_e(color1: &Color, color2: &Color) -> f64 {
    let lab1 = color1.vec_of(ColorSpace::Lab);
    let lab2 = color2.vec_of(ColorSpace::Lab);

    let [l1, a1, b1] = [lab1[0], lab1[1], lab1[2]];
    let [l2, a2, b2] = [lab2[0], lab2[1], lab2[2]];

    let avg_l = (l1 + l2) / 2.0;

    let c1 = (a1.powi(2) + b1.powi(2)).sqrt();
    let c2 = (a2.powi(2) + b2.powi(2)).sqrt();

    let avg_c = (c1 + c2) / 2.0;

    let g = 0.5 * (1.0 - (avg_c.powi(7) / (avg_c.powi(7) + (25.0_f64).powi(7))).sqrt());

    let a1p = (1.0 + g) * a1;
    let a2p = (1.0 + g) * a2;

    let c1p = (a1p.powi(2) + b1.powi(2)).sqrt();
    let c2p = (a2p.powi(2) + b2.powi(2)).sqrt();

    let avg_cp = (c1p + c2p) / 2.0;

    let arctan1 = b1.atan2(a1p).to_degrees();
    let arctan2 = b2.atan2(a2p).to_degrees();

    let h1p = if arctan1 >= 0.0 {
        arctan1
    } else {
        arctan1 + 360.0
    };
    let h2p = if arctan2 >= 0.0 {
        arctan2
    } else {
        arctan2 + 360.0
    };

    let avg_hp = if (h1p - h2p).abs() > 180.0 {
        (h1p + h2p + 360.0) / 2.0
    } else {
        (h1p + h2p) / 2.0
    };

    let t = 1.0 - 0.17 * (avg_hp - 30.0).to_radians().cos()
        + 0.24 * (2.0 * avg_hp).to_radians().cos()
        + 0.32 * (3.0 * avg_hp + 6.0).to_radians().cos()
        - 0.2 * (4.0 * avg_hp - 63.0).to_radians().cos();

    let mut delta_hp = if (h2p - h1p).abs() <= 180.0 {
        h2p - h1p
    } else if h2p <= h1p {
        h2p - h1p + 360.0
    } else {
        h2p - h1p - 360.0
    };
    delta_hp = 2.0 * (c1p * c2p).sqrt() * (delta_hp / 2.0).to_radians().sin();

    let delta_l = l2 - l1;
    let delta_cp = c2p - c1p;

    let sl = 1.0 + (0.015 * (avg_l - 50.0).powi(2)) / (20.0 + (avg_l - 50.0).powi(2)).sqrt();
    let sc = 1.0 + 0.045 * avg_cp;
    let sh = 1.0 + 0.015 * avg_cp * t;

    let delta_theta = 30.0 * (-((avg_hp - 275.0) / 25.0).powi(2)).exp();

    let rc = 2.0 * (avg_cp.powi(7) / (avg_cp.powi(7) + (25.0_f64).powi(7))).sqrt();
    let rt = -rc * (2.0 * delta_theta).to_radians().sin();

    let kl = 1.0;
    let kc = 1.0;
    let kh = 1.0;

    let result = ((delta_l / (kl * sl)).powi(2)
        + (delta_cp / (kc * sc)).powi(2)
        + (delta_hp / (kh * sh)).powi(2)
        + rt * (delta_cp / (kc * sc)) * (delta_hp / (kh * sh)))
        .sqrt();

    result.min(100.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn is_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.000001
    }

    #[test]
    fn test_distance() {
        let color1 = color!(#fefe0e);
        let color2 = color!(#fff);

        let d = delta_e(&color1, &color2);
        assert!(is_equal(d, 30.165629067733235));

        let color1 = color!(#ededee);
        let color2 = color!(#edeeed);

        let d = delta_e(&color1, &color2);
        assert!(is_equal(d, 1.2364506278716838));

        let color1 = color!(#e0e0ee);
        let color2 = color!(#e0eee0);

        let d = delta_e(&color1, &color2);
        assert!(is_equal(d, 14.618185117695797));

        let color1 = color!(#fff);
        let color2 = color!(#000);

        let d = delta_e(&color1, &color2);
        assert_eq!(d, 100.0);
    }
}
