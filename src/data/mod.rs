pub mod alpha_hex_map;
pub mod chinese_color;
pub mod w3cx11;

pub(crate) fn hex_str_of(color_name: &str) -> Option<&'static str> {
    let hex_str = crate::W3CX11.get(color_name);
    match hex_str {
        Some(hex_str) => Some(hex_str),
        None => {
            let res = crate::CHINESE_COLOR.get(color_name);
            match res {
                Some(hex_str) => Some(hex_str),
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_color_name() {
        let color_name = hex_str_of("yellow");
        assert_eq!(color_name, Some("#ffff00"));

        let color_name = hex_str_of("水绿");
        assert_eq!(color_name, Some("#8cc269"));

        let color_name = hex_str_of("没有的颜色");
        assert_eq!(color_name, None);
    }
}
