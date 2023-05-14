#[macro_export]
macro_rules! color_args {
    () => { vec![] };
    ($arg:expr) => { vec![$arg] };
    ($arg:expr, $($rest:tt)*) => {
        {
            let mut t = vec![$arg];
            t.append(&mut $crate::color_args!($($rest)*));
            t
        }
    };
}

#[macro_export]
macro_rules! color {
    ($color_space:ident, $($args:tt)*) => {
        {
          match stringify!($color_space).into() {
              $crate::ColorSpace::RGB => {
                  let args = $crate::color_args!($($args)*);
                  let r = args[0];
                  let g = args[1];
                  let b = args[2];
                  $crate::Color::from_rgb(r, g, b).unwrap()
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
    }
}
