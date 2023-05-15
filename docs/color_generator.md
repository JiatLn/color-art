# Color Generator

## random

Generate a random color.

```rust
use color_art::Color;

let color = Color::random();
color.hex(); // "#f2e6c7"
```

## mix

Mix two colors.

```rust
use color_art::Color;
use std::str::FromStr;

let color1 = Color::from_str("#998099").unwrap();
let color2 = Color::from_str("midnightblue").unwrap();
let color3 = Color::mix(&color1, &color2, 0.5).unwrap();
color3.hex(); // "#594d85"
```

## average

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

## blend

Blend two colors with a blending mode.

Current supported blending modes:

- Normal
- Multiply
- Screen
- Overlay
- Darken
- Lighten
- ColorDodge
- ColorBurn
- HardLight
- SoftLight
- Difference
- Exclusion

```rust
use color_art::{Color, BlendMode, blend};
use std::str::FromStr;

let color1 = Color::from_str("#4cbbfc").unwrap();
let color2 = Color::from_str("#eeee22").unwrap();

let blended_color = blend(&color1, &color2, BlendMode::Overlay);
blended_color.hex() // "#8ef6fa"
```
