use crate::{Color, ColorSpace};

pub enum BlendMode {
    Mutiply,
    Darken,
    Lighten,
    Screen,
    Overlay,
    Burn,
    Dodge,
}

/// Blends two colors using RGB channel-wise blend functions.
///
/// blend mode formulas taken from [lets-learn-math-photoshop-blend-modes](http://www.venture-ware.com/kevin/coding/lets-learn-math-photoshop-blend-modes/)
pub fn blend(bottom_color: &Color, top_color: &Color, mode: BlendMode) -> Color {
    let bottom_vec = bottom_color.space(ColorSpace::RGB).unwrap();
    let top_vev = top_color.space(ColorSpace::RGB).unwrap();
    let zip_vec = bottom_vec.iter().zip(top_vev.iter());
    let v: Vec<_> = match mode {
        BlendMode::Mutiply => zip_vec.map(|(&a, &b)| mutiply(a, b)).collect(),
        BlendMode::Darken => zip_vec.map(|(&a, &b)| a.max(b)).collect(),
        BlendMode::Lighten => zip_vec.map(|(&a, &b)| a.min(b)).collect(),
        BlendMode::Screen => zip_vec.map(|(&a, &b)| screen(a, b)).collect(),
        BlendMode::Overlay => zip_vec.map(|(&a, &b)| overlay(a, b)).collect(),
        BlendMode::Burn => zip_vec.map(|(&a, &b)| burn(a, b)).collect(),
        BlendMode::Dodge => zip_vec.map(|(&a, &b)| dodge(a, b)).collect(),
    };
    let r = v[0];
    let g = v[1];
    let b = v[2];
    Color::new(r, g, b, 1.0)
}

fn mutiply(a: f64, b: f64) -> f64 {
    a * b / 255.
}

fn screen(a: f64, b: f64) -> f64 {
    255. * (1. - (1. - a / 255.) * (1. - b / 255.))
}

fn overlay(a: f64, b: f64) -> f64 {
    if b < 128. {
        2. * a * b / 255.
    } else {
        255. * (1. - 2. * (1. - a / 255.) * (1. - b / 255.))
    }
}

fn burn(a: f64, b: f64) -> f64 {
    255. * (1. - (1. - b / 255.) / (a / 255.))
}

fn dodge(a: f64, b: f64) -> f64 {
    if a == 255. {
        255.
    } else {
        let a = 255. * (b / 255.) / (1. - a / 255.);
        a.min(255.)
    }
}
