/// Simplifies the hex code to a short hex code if possible.
pub(crate) fn simplify_hex(hex: String) -> String {
    let hex_len = hex.len();
    if hex_len == 7 || hex_len == 9 {
        let r1 = hex.chars().nth(1).unwrap();
        let r2 = hex.chars().nth(2).unwrap();
        let g1 = hex.chars().nth(3).unwrap();
        let g2 = hex.chars().nth(4).unwrap();
        let b1 = hex.chars().nth(5).unwrap();
        let b2 = hex.chars().nth(6).unwrap();

        if hex_len == 9 {
            let a1 = hex.chars().nth(7).unwrap();
            let a2 = hex.chars().nth(8).unwrap();

            return if r1 == r2 && g1 == g2 && b1 == b2 && a1 == a2 {
                format!("#{}{}{}{}", r1, g1, b1, a1)
            } else {
                hex
            };
        }

        if r1 == r2 && g1 == g2 && b1 == b2 {
            format!("#{}{}{}", r1, g1, b1)
        } else {
            hex
        }
    } else {
        hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_hex() {
        let color = "#00ff00".to_string();
        assert_eq!(simplify_hex(color), "#0f0".to_string());

        let color = "#abcdef".to_string();
        assert_eq!(simplify_hex(color), "#abcdef".to_string());

        let color = "#00ff0033".to_string();
        assert_eq!(simplify_hex(color), "#0f03".to_string());

        let color = "#abcdef33".to_string();
        assert_eq!(simplify_hex(color), "#abcdef33".to_string());
    }
}
