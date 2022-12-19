use color_art::Color;
use std::str::FromStr;

fn main() {
    let color = Color::from_str("hsl(300, 100%, 50%)");

    match color {
        Ok(color) => {
            println!("{:#?}", color);

            let hex = color.hex();
            println!("{}", hex);

            let rgb = color.rgb();
            println!("{}", rgb);

            let rgba = color.rgba();
            println!("{}", rgba);

            let hsl = color.hsl();
            println!("{}", hsl);
        }
        Err(err) => println!("error: {}", err),
    }
}
