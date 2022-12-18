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
- Hex

For example:

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("rgb(255, 255, 0)").unwrap();
let color = Color::from_str("rgba(255, 255, 0, 0.5)").unwrap();
let color = Color::from_str("#ffff00").unwrap();
```

#### Color Space

Stringify a color to a string. You can use the `hex`, `rgb`, `rgba` method to stringify a color to a string. For example:

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("rgb(255, 255, 0)").unwrap();
color.hex(); // "#ffff00"
color.rgb(); // "rgb(255, 255, 0)"
color.rgba(); // "rgba(255, 255, 0, 1)"
```

#### random

You can use the `random` method to construct a random color.

```rust
use color_art::Color;

let color = Color::random();
```

## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
