use crate::Color;

/// Computes the WCAG contrast ratio between two colors
///
/// Reference: [W3C contrast ratio](https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef)
///
/// **Tips**: A minimum contrast of 4.5:1 is [recommended](http://www.w3.org/TR/WCAG20-TECHS/G18.html) to ensure that text is still readable against a background color.
///
/// # Example
///
/// ```
/// use color_art::{color, contrast_ratio};
///
/// let color1 = color!(#fefe0e);
/// let color2 = color!(#fff);
/// let contrast = contrast_ratio(&color1, &color2);
///
/// assert_eq!(contrast, 1.0826287103122008);
pub fn contrast_ratio(color1: &Color, color2: &Color) -> f64 {
    let l1 = color1.luminance();
    let l2 = color2.luminance();
    let l_max = l1.max(l2);
    let l_min = l1.min(l2);
    (l_max + 0.05) / (l_min + 0.05)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_distance() {
        let color1 = color!(#000);
        let color2 = color!(#fff);

        let contrast = contrast_ratio(&color1, &color2);
        assert_eq!(contrast, 21.0);

        let color1 = color!(#fefe0e);
        let color2 = color!(#fff);

        let contrast = contrast_ratio(&color1, &color2);
        assert_eq!(contrast, 1.0826287103122008);

        let color1 = Color::from_name("purple").unwrap();
        let color2 = Color::from_name("pink").unwrap();

        let contrast = contrast_ratio(&color1, &color2);
        assert_eq!(contrast, 6.124225406859997);
    }
}
