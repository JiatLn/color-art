pub mod alpha_hex_map;
pub mod chinese_color;
pub mod w3cx11;

pub(crate) fn hex_of_name(color_name: &str) -> Option<&'static str> {
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

pub(crate) fn name_of_hex(hex_str: &str) -> Option<&'static str> {
    let result = crate::W3CX11
        .clone()
        .into_iter()
        .find(|(_k, v)| *v == hex_str)
        .map(|(k, _v)| k);

    match result {
        Some(name) => Some(name),
        None => {
            let result = crate::CHINESE_COLOR
                .clone()
                .into_iter()
                .find(|(_k, v)| *v == hex_str)
                .map(|(k, _v)| k);
            match result {
                Some(name) => Some(name),
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
        let color_name = hex_of_name("yellow");
        assert_eq!(color_name, Some("#ffff00"));

        let color_name = hex_of_name("水绿");
        assert_eq!(color_name, Some("#8cc269"));

        let color_name = hex_of_name("没有的颜色");
        assert_eq!(color_name, None);
    }
}
