#[macro_export]
macro_rules! color_args {
    () => { vec![] };
    ($arg:expr) => { vec![$arg] };
    (
        $arg:expr,
        $($rest:tt)*
    ) => {
        {
            let mut t = vec![$arg];
            t.append(&mut $crate::color_args!($($rest)*));
            t
        }
    };
}

#[macro_export]
macro_rules! color {
    (#$hex:expr) => {
        {
            let hex = format!("#{}", stringify!($hex));
            $crate::Color::from_hex(&hex).unwrap()
        }
    };
    (
        $color_space:ident,
        $($args:tt)*
    ) => {
        {
            let args = $crate::color_args!($($args)*);
            match stringify!($color_space).into() {
                $crate::ColorSpace::RGB => {
                    let r = args[0] as u8;
                    let g = args[1] as u8;
                    let b = args[2] as u8;
                    $crate::Color::from_rgb(r, g, b).unwrap()
                }
                $crate::ColorSpace::HSL => {
                    let h = args[0] as f64;
                    let s = args[1] as f64;
                    let l = args[2] as f64;
                    $crate::Color::from_hsl(h, s, l).unwrap()
                },
                $crate::ColorSpace::HSV => {
                    let h = args[0] as f64;
                    let s = args[1] as f64;
                    let v = args[2] as f64;
                    $crate::Color::from_hsv(h, s, v).unwrap()
                }
                _ => todo!("Add more color spaces to color! macro"),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    #[test]
    fn test_color_args_macro() {
        let args = color_args!(255, 128, 0);
        assert_eq!(args, vec![255, 128, 0]);

        let args = color_args!(255.0, 255.0, 0.0);
        assert_eq!(args, vec![255.0, 255.0, 0.0]);
    }

    #[test]
    fn test_color_macro() {
        let color = color!(rgb, 255, 255, 0);
        assert_eq!(color.rgb(), "rgb(255, 255, 0)");

        let color = color!(hsl, 60.0, 1.0, 0.5);
        assert_eq!(color.hsl(), "hsl(60, 100%, 50%)");

        let color = color!(hsv, 60.0, 1.0, 1.0);
        assert_eq!(color.hsv(), "hsv(60, 100%, 100%)");

        let color = color!(#f00);
        assert_eq!(color.hex(), "#ff0000");

        let color = color!(#abcdef);
        assert_eq!(color.hex(), "#abcdef");
    }
}
