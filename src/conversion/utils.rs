// D65 standard referent
pub(crate) const XN: f64 = 0.950470;
pub(crate) const YN: f64 = 1.0;
pub(crate) const ZN: f64 = 1.088830;

pub(crate) const T0: f64 = 4.0 / 29.0;
pub(crate) const T1: f64 = 6.0 / 29.0;
pub(crate) const T2: f64 = 3.0 * T1 * T1;
pub(crate) const T3: f64 = T1 * T1 * T1;

pub(crate) fn xyz_lab(t: f64) -> f64 {
    if t > T3 {
        t.powf(1.0 / 3.0)
    } else {
        t / T2 + T0
    }
}

pub(crate) fn rgb_xyz(r: f64) -> f64 {
    if r <= 0.04045 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    }
}

pub(crate) fn lab_xyz(t: f64) -> f64 {
    if t > T1 {
        t * t * t
    } else {
        T2 * (t - T0)
    }
}

pub(crate) fn xyz_rgb(r: f64) -> f64 {
    if r <= 0.00304 {
        12.92 * r
    } else {
        1.055 * r.powf(1.0 / 2.4) - 0.055
    }
}
