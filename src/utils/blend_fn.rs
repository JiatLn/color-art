pub(crate) fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub(crate) fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

pub(crate) fn normal(_a: f64, b: f64) -> f64 {
    b
}

pub(crate) fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub(crate) fn screen(a: f64, b: f64) -> f64 {
    a + b - a * b
}

pub(crate) fn overlay(a: f64, b: f64) -> f64 {
    if a <= 0.5 {
        multiply(a, 2. * b)
    } else {
        screen(a, 2. * b - 1.)
    }
}

pub(crate) fn burn(a: f64, b: f64) -> f64 {
    if a == 1. {
        1.
    } else if b == 0. {
        0.
    } else {
        1. - min(1., (1. - a) / b)
    }
}

pub(crate) fn dodge(a: f64, b: f64) -> f64 {
    if a == 0. {
        0.
    } else if b == 1. {
        1.
    } else {
        (a / (1. - b)).min(1.)
    }
}

pub(crate) fn hard_light(a: f64, b: f64) -> f64 {
    overlay(b, a)
}

pub(crate) fn soft_light(a: f64, b: f64) -> f64 {
    if b <= 0.5 {
        a - (1. - 2. * b) * a * (1. - a)
    } else {
        let d = |a: f64| {
            if a <= 0.25 {
                ((16. * a - 12.) * a + 4.) * a
            } else {
                a.sqrt()
            }
        };
        a + (2. * b - 1.) * (d(a) - a)
    }
}

pub(crate) fn difference(a: f64, b: f64) -> f64 {
    (a - b).abs()
}

pub(crate) fn exclusion(a: f64, b: f64) -> f64 {
    a + b - 2. * a * b
}
