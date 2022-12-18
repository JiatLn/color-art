use color_art::Color;
use std::str::FromStr;

fn main() {
    let color = Color::from_str("#ffff00");

    match color {
        Ok(color) => {
            let hex = color.hex();
            println!("{}", hex);

            let rgb = color.rgb();
            println!("{}", rgb);

            let rgba = color.rgba();
            println!("{}", rgba);
        }
        Err(err) => println!("error: {}", err),
    }
}
