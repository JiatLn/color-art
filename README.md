# Color Art

[<img alt="github" src="https://img.shields.io/badge/github-JiatLn/color_art-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/JiatLn/color-art)
[<img alt="crates.io" src="https://img.shields.io/crates/v/color_art.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/color-art)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-color_art-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/color_art)


> A rust crate for working with colors and color spaces.

## Documentation

See [Color Art](https://docs.rs/color-art).


## Usage

### Add Dependency

```toml
[dependencies]
color-art = "0.2"
```

### Color generator

You can use the `from_str` method to construct a color from a string. 

Currently supported color formats:

- RGB
- RGBA
- HEX
- HSL
- HSV
- HWB
- CMYK
- Named color from [*w3cx11*](http://www.w3.org/TR/css3-color/#svg-color)

For example:

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("rgb(255, 255, 0)").unwrap();
let color = Color::from_str("rgba(255, 255, 0, 0.5)").unwrap();
let color = Color::from_str("#ffff00").unwrap();
let color = Color::from_str("hsl(60, 100%, 50%)").unwrap();
let color = Color::from_str("hsv(60, 100%, 100%)").unwrap();
let color = Color::from_str("hwb(60, 0%, 0%)").unwrap();
let color = Color::from_str("cmyk(0%, 0%, 100%, 0%)").unwrap();
let color = Color::from_str("yellow").unwrap();
```

Other color generator methods:

- [random](./docs/color_generator.md#random) - Generate a random color.
- [mix](./docs/color_generator.md#mix) - Mix two colors.
- [average](./docs/color_generator.md#average) - Average a list of colors.

### Color conversion

#### Stringify a color

Stringify a color to a string. You can use the `hex`, `rgb`, `rgba`, `hsl`, `hsv`, `hwb`, `name` method to stringify a color to a string. For example:

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#ffff00").unwrap();
color.hex(); // "#ffff00"
color.rgb(); // "rgb(255, 255, 0)"
color.rgba(); // "rgba(255, 255, 0, 1)"
color.hsl(); // "hsl(60, 100%, 50%)"
color.hsv(); // "hsv(60, 100%, 100%)"
color.hwb(); // "hwb(60, 0%, 0%)"
color.cmyk(); // "cmyk(0%, 0%, 100%, 0%)"
color.name(); // "yellow"
```

### Color Channel

You can use the `red`, `green`, `blue`, `alpha`, `hue`, `saturation`, `lightness`, `hsv_hue`, `hsv_saturation`, `hsv_value`, `luma`, `luminance` method to extract the color channel.

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#abcdef").unwrap();

color.red(); // 171
color.green(); // 205
color.blue(); // 239
color.alpha(); // 1.0

color.hue(); // 210.0
color.saturation(); // 0.68
color.lightness(); // 0.8

color.hsv_hue(); // 210.0
color.hsv_saturation(); // 0.28
color.hsv_value(); // 0.94

color.luma(); // 0.59
color.luminance(); // 0.79
```

### Color Operation

- [darken](./docs/color_operation.md#darken) - Darken a color.
- [lighten](./docs/color_operation.md#lighten) - Lighten a color.
- [saturate](./docs/color_operation.md#saturate) - Saturate a color.
- [desaturate](./docs/color_operation.md#desaturate) - Desaturate a color.
- [greyscale](./docs/color_operation.md#greyscale) - Greyscale a color.
- [fade](./docs/color_operation.md#fade) - Fade a color.
- [fade in](./docs/color_operation.md#fade_in) - Fade in a color.
- [fade out](./docs/color_operation.md#fade_out) - Fade out a color.
- [spin](./docs/color_operation.md#spin) - Spin a color.
- [mix](./docs/color_operation.md#mix) - Mix with another color.
- [tint](./docs/color_operation.md#tint) - Tint a color.
- [shade](./docs/color_operation.md#shade) - Shade a color.

More details, please refer to [Color Operation](./docs/color_operation.md).

---


Made with ?????? by [JiatLn](https://github.com/JiatLn).

## License

[MIT](./LICENSE) License ?? 2022-Present [JiatLn](https://github.com/JiatLn)
