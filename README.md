# Color Art
A rust crate for working with colors and color spaces.

More technical details, please refer to [RFC: Construct Color from Color Space](./rfcs/001-Construct%20Color%20from%20Color%20Space.md).

## Usage

### Add Dependency

```toml
[dependencies]
color-art = "0.1"
```

### Color

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

#### Color Space

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

#### random

You can use the `random` method to construct a random color.

```rust
use color_art::Color;

let color = Color::random();
```

#### Average a list of colors

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

## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
