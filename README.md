<br>

<p align="center">
<img src="./graphs/logo.png" alt="ColorArt - A rust crate for working with colors and color spaces." width="120">
</p>

<h1 align="center">Color Art</h1>

<p align="center">
A rust crate for working with colors and color spaces.
</p>

<p align="center">
  <a href="https://github.com/JiatLn/color-art" target="_blank">
    <img alt="github" src="https://img.shields.io/badge/github-JiatLn/color_art-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">
  </a>
  <a href="https://crates.io/crates/color-art" target="_blank">
    <img alt="crates.io" src="https://img.shields.io/crates/v/color_art.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">
  </a>
  <a href="https://docs.rs/color_art" target="_blank">
    <img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-color_art-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">
  </a>
</p>

<br>

## Documentation

See [Color Art](https://color-art.netlify.app).


## Usage

### Add Dependency

```toml
[dependencies]
color-art = "0.3"
```

### Color generator

#### Create color from string

You can use the `from_str` method to construct a color from a string. 

<details>
<summary>Currently supported color formats</summary>
<br>
<ul>
  <li><code>rgb</code> / <code>rgba</code></li>
  <li><code>hex</code></li>
  <li><code>hsl</code> / <code>hsla</code></li>
  <li><code>hsv</code></li>
  <li><code>hsi</code></li>
  <li><code>hwb</code></li>
  <li><code>cmyk</code></li>
  <li><code>xyz</code></li>
  <li><code>yuv</code></li>
  <li><code>YCbCr</code></li>
  <li><code>lab</code></li>
  <li><code>named color</code></li>
</ul>
</details>

<details>
<summary>For example</summary>
<br>

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("rgb(255, 255, 0)").unwrap();
let color = Color::from_str("rgba(255, 255, 0, 0.5)").unwrap();
let color = Color::from_str("#ffff00").unwrap();
let color = Color::from_str("hsl(60, 100%, 50%)").unwrap();
let color = Color::from_str("hsla(60, 100%, 50%, 0.6)").unwrap();
let color = Color::from_str("hsv(60, 100%, 100%)").unwrap();
let color = Color::from_str("hsi(60, 100%, 66.67%)").unwrap();
let color = Color::from_str("hwb(60, 0%, 0%)").unwrap();
let color = Color::from_str("cmyk(0%, 0%, 100%, 0%)").unwrap();
let color = Color::from_str("xyz(0.932231, 0.975339, 0.502949)").unwrap();
let color = Color::from_str("yuv(0.886, -0.4359, 0.1)").unwrap();
let color = Color::from_str("YCbCr(225.93, 0.5755, 148.7269)").unwrap();
let color = Color::from_str("lab(97.14, -21.55, 94.48)").unwrap();
let color = Color::from_str("yellow").unwrap();
```

</details>

#### Create color from number

You can use the `from_num` method to construct a color from a number.

For example:

```rust
use color_art::Color;

let color = Color::from_num(16776960).unwrap();
let color = Color::from_num(0xffff00).unwrap();
```

#### Create color from name

You can use the `from_name` method to construct a color from a name.

For example:

```rust
use color_art::Color;

let color = Color::from_name("yellow").unwrap();
```

#### Create color from color space

You can use the `from_<color_space>` method to construct a color from a color space.

Currently supported color spaces:

- `rgb` 
- `rgba`
- `hsl`
- `hsv`
- `cmyk`
- `hex`

More color spaces will be supported in the future.

For example:

```rust
use color_art::Color;

let color = Color::from_rgb(255, 255, 0).unwrap();
let color = Color::from_rgba(255, 255, 0, 0.5).unwrap();
let color = Color::from_hsl(60.0, 1.0, 0.5).unwrap();
let color = Color::from_hsv(60.0, 1.0, 1.0).unwrap();
let color = Color::from_cmyk(0.0, 0.0, 1.0, 0.0).unwrap();
let color = Color::from_hex("#ffff00").unwrap();
```

More examples can be found in [Construct from color spaces](https://color-art.netlify.app/construct-a-color/from-space).

#### Other color generator methods

- [random](./docs/color_generator.md#random) - Generate a random color.
- [mix](./docs/color_generator.md#mix) - Mix two colors.
- [blend](./docs/color_generator.md#blend) - Blend two colors with a blending mode.
- [average](./docs/color_generator.md#average) - Average a list of colors.

### Color conversion

#### Stringify a color

Stringify a color to a string.

> Refer to [Construct from string](https://color-art.netlify.app/construct-a-color/from-string.html).

</details>

### Color Channels

Extract the color channels.

> Refer to [Color Channels](https://color-art.netlify.app/api/channels.html).

### Color Operations

Color operation functions.

> Refer to [Color Operations](https://color-art.netlify.app/api/operations.html).

---


Made with ❤️ by [JiatLn](https://github.com/JiatLn).

## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
