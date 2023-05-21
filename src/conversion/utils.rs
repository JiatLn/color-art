use crate::utils::multiply_matrices;

// standard white points, defined by 4-figure CIE x,y chromaticities
pub(crate) const D50: [f64; 3] = [0.3457 / 0.3585, 1.0, (1.0 - 0.3457 - 0.3585) / 0.3585];
// pub(crate) const D65: [f64; 3] = [0.3127 / 0.329, 1.0, (1.0 - 0.3127 - 0.329) / 0.329];

static RGB2XYZ_MATRIX: [[f64; 3]; 3] = [
    [506752.0 / 1228815.0, 87881.0 / 245763.0, 12673.0 / 70218.0],
    [87098.0 / 409605.0, 175762.0 / 245763.0, 12673.0 / 175545.0],
    [7918.0 / 409605.0, 87881.0 / 737289.0, 1001167.0 / 1053270.0],
];

static XYZ2RGB_MATRIX: [[f64; 3]; 3] = [
    [12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
    [-851781.0 / 878810.0, 1648619.0 / 878810.0, 36519.0 / 878810.0],
    [705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0],
];

/// convert an array of linear-light sRGB values to CIE XYZ
///
/// using sRGB's own white, D65 (no chromatic adaptation)
pub(crate) fn lin_srgb_to_xyz(rgb: (f64, f64, f64)) -> (f64, f64, f64) {
    let rgb = vec![vec![rgb.0], vec![rgb.1], vec![rgb.2]];
    let rgb2xyz_matrix = RGB2XYZ_MATRIX.map(|v| v.to_vec()).to_vec();
    let xyz: Vec<Vec<f64>> = multiply_matrices(rgb2xyz_matrix, rgb);
    (xyz[0][0], xyz[1][0], xyz[2][0])
}

/// convert XYZ to linear-light sRGB
pub(crate) fn xyz_to_lin_srgb(xyz: (f64, f64, f64)) -> (f64, f64, f64) {
    let xyz = vec![vec![xyz.0], vec![xyz.1], vec![xyz.2]];
    let xyz2rgb_matrix = XYZ2RGB_MATRIX.map(|v| v.to_vec()).to_vec();
    let rgb = multiply_matrices(xyz2rgb_matrix, xyz);
    (rgb[0][0], rgb[1][0], rgb[2][0])
}

pub(crate) fn lin_srgb(rgb: (f64, f64, f64)) -> (f64, f64, f64) {
    let rgb: Vec<f64> = [rgb.0, rgb.1, rgb.2]
        .iter()
        .map(|&v| {
            let sign = v.signum();
            let abs = v.abs();
            if abs < 0.04045 {
                v / 12.92
            } else {
                sign * ((abs + 0.055) / 1.055).powf(2.4)
            }
        })
        .collect();
    (rgb[0], rgb[1], rgb[2])
}

pub(crate) fn xyz2lab(xyz: Vec<f64>) -> Vec<f64> {
    const E: f64 = 216.0 / 24389.0; // 6^3/29^3
    const K: f64 = 24389.0 / 27.0; // 29^3/3^3
    let xyz: Vec<_> = xyz
        .iter()
        .zip(D50.iter())
        .map(|(v1, v2)| v1 / v2)
        .map(|v| if v > E { v.cbrt() } else { (K * v + 16.0) / 116.0 })
        .collect();
    vec![116.0 * xyz[1] - 16.0, 500.0 * (xyz[0] - xyz[1]), 200.0 * (xyz[1] - xyz[2])]
}

pub(crate) fn d65_to_d50(xyz: Vec<f64>) -> Vec<f64> {
    let m = [
        [1.0479298208405488, 0.022946793341019088, -0.05019222954313557],
        [0.029627815688159344, 0.990434484573249, -0.01707382502938514],
        [-0.009243058152591178, 0.015055144896577895, 0.7518742899580008],
    ];
    let m = m.map(|v| v.to_vec()).to_vec();
    let xyz = xyz
        .iter()
        .map(|&v| vec![v])
        .collect();
    multiply_matrices(m, xyz)
        .iter()
        .map(|v| v[0])
        .collect()
}

pub(crate) fn d50_to_d65(xyz: Vec<f64>) -> Vec<f64> {
    let m = [
        [0.9554734527042182, -0.023098536874261423, 0.0632593086610217],
        [-0.028369706963208136, 1.0099954580058226, 0.021041398966943008],
        [0.012314001688319899, -0.020507696433477912, 1.3303659366080753],
    ];
    let m = m.map(|v| v.to_vec()).to_vec();
    let xyz = xyz
        .iter()
        .map(|&v| vec![v])
        .collect();
    multiply_matrices(m, xyz)
        .iter()
        .map(|v| v[0])
        .collect()
}

/// Convert Lab to D50-adapted XYZ
///
/// <http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html>
pub(crate) fn lab2xyz(lab: Vec<f64>) -> Vec<f64> {
    const K: f64 = 24389.0 / 27.0; // 29^3/3^3
    const E: f64 = 216.0 / 24389.0; // 6^3/29^3

    let f1 = (lab[0] + 16.0) / 116.0;
    let f0 = lab[1] / 500.0 + f1;
    let f2 = f1 - lab[2] / 200.0;

    let x = if f0.powi(3) > E { f0.powi(3) } else { (116.0 * f0 - 16.0) / K };
    let y = if lab[0] > K * E { ((lab[0] + 16.0) / 116.0).powi(3) } else { lab[0] / K };
    let z = if f2.powi(3) > E { f2.powi(3) } else { (116.0 * f2 - 16.0) / K };

    vec![x, y, z]
        .iter()
        .zip(D50.iter())
        .map(|(v1, v2)| v1 * v2)
        .collect()
}
