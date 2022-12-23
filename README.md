# Color Art
A rust crate for working with colors and color spaces.

More technical details, please refer to [RFC: Construct Color from Color Space](./rfcs/001-Construct%20Color%20from%20Color%20Space.md).

## Usage

### Add Dependency

```toml
[dependencies]
color-art = "0.1"
```

### Color generation

#### Color Space

Construct a color from a string. You can use the `from_str` method to construct a color from a string. Currently supported color spaces are:

- RGB
- RGBA
- HEX
- HSL
- HSV
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
let color = Color::from_str("yellow").unwrap();
```

#### Random

You can use the `random` method to construct a random color.

```rust
use color_art::Color;

let color = Color::random();
```

### Color conversion

#### Stringify a color

Stringify a color to a string. You can use the `hex`, `rgb`, `rgba`, `hsl`, `hsv`, `name` method to stringify a color to a string. For example:

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#ffff00").unwrap();
color.hex(); // "#ffff00"
color.rgb(); // "rgb(255, 255, 0)"
color.rgba(); // "rgba(255, 255, 0, 1)"
color.hsl(); // "hsl(60, 100%, 50%)"
color.hsv(); // "hsv(60, 100%, 100%)"
color.name(); // "yellow"
```

### Color Channel

You can use the `red`, `green`, `blue`, `hue`, `saturation`, `lightness`, `alpha` method to extract the color channel.

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#ffff00").unwrap();
color.red(); // 255
color.green(); // 255
color.blue(); // 0
color.hue(); // 60.0
color.saturation(); // 1.0
color.lightness(); // 0.5
color.alpha(); // 1.0
```


### Color Operation

#### average

You can use the `average` method to average a list of colors.

```rust
use color_art::Color;
use std::str::FromStr;

let colors = vec![
    Color::from_str("#ffff00").unwrap(),
    Color::from_str("#ff0000").unwrap(),
    Color::from_str("#0000ff").unwrap(),
];

let averaged_color = Color::average(&colors);
averaged_color.hex(); // "#aa5555"
```

#### darken

Decrease the lightness of a color in the HSL color space by an absolute amount.

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#ffff00").unwrap();
let darkened_color = color.darken(0.2).unwrap();
darkened_color.hex(); // "#999900"
```

#### lighten

Increase the lightness of a color in the HSL color space by an absolute amount.

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#ffff00").unwrap();
let lightened_color = color.lighten(0.2).unwrap();
lightened_color.hex(); // "#ffff66"
```

## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
