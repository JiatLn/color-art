use color_art::Color;

fn main() {
    let color = Color::random();

    println!("{:#?}", color);

    let hex = color.hex();
    println!("{}", hex);

    let rgb = color.rgb();
    println!("{}", rgb);

    let rgba = color.rgba();
    println!("{}", rgba);

    let hsl = color.hsl();
    println!("{}", hsl);

    let hsv = color.hsv();
    println!("{}", hsv);
}
