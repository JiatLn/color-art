pub fn rgb2hex(color: (f64, f64, f64)) -> String {
    let (r, g, b) = color;
    let r = r.round() as u8;
    let g = g.round() as u8;
    let b = b.round() as u8;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn hex2rgb(hex: &str) -> (f64, f64, f64) {
    let mut hex = String::from(hex);
    let len = hex.len();
    if len == 4 {
        let mut hex_new = String::from("#");
        for c in hex[1..].chars() {
            hex_new.push(c);
            hex_new.push(c);
        }
        hex = hex_new;
    }

    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();

    (r as f64, g as f64, b as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex2rgb() {
        let hex = "#ffffff";
        let rgb = hex2rgb(hex);
        assert_eq!(rgb, (255.0, 255.0, 255.0));

        let hex = "#000";
        let rgb = hex2rgb(hex);
        assert_eq!(rgb, (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_rgb2hex() {
        let rgb = (255.0, 255.0, 255.0);
        let hex = rgb2hex(rgb);
        assert_eq!(hex, "#ffffff");

        let rgb = (0.0, 0.0, 0.0);
        let hex = rgb2hex(rgb);
        assert_eq!(hex, "#000000");

        let rgb = (0.0, 128.0, 128.4);
        let hex = rgb2hex(rgb);
        assert_eq!(hex, "#008080");
    }
}
