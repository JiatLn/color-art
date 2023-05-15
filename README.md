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
color-art = "0.2"
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

You can use the `hex`, `rgb`, `rgba`, `hsl`, `hsla`, `hsv`, `hsi`, `hwb`, `cmyk`, `xyz`, `yuv`, `YCbCr`, `lab`, `name` method to stringify a color to a string.

<details>
<summary>For example</summary>
<br>
  
```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("#ffff00").unwrap();
color.hex(); // "#ffff00"
color.rgb(); // "rgb(255, 255, 0)"
color.rgba(); // "rgba(255, 255, 0, 1)"
color.hsl(); // "hsl(60, 100%, 50%)"
color.hsla(); // "hsl(60, 100%, 50%, 1)"
color.hsv(); // "hsv(60, 100%, 100%)"
color.hsi(); // "hsi(60, 100%, 66.67%)"
color.hwb(); // "hwb(60, 0%, 0%)"
color.cmyk(); // "cmyk(0%, 0%, 100%, 0%)"
color.xyz(); // "xyz(0.932231, 0.975339, 0.502949)"
color.yuv(); // "yuv(0.886, -0.4359, 0.1)"
color.ycbcr(); // "YCbCr(225.93, 0.5755, 148.7269)"
color.name(); // "yellow"
color.lab(); // "lab(97.14, -21.55, 94.48)"
```

</details>

### Color Channel

You can use the `red`, `green`, `blue`, `alpha`, `hue`, `saturation`, `lightness`, `hsv_hue`, `hsv_saturation`, `hsv_value`, `luma`, `luminance`, `gray` method to extract the color channel.

<details>
<summary>For example</summary>
<br>
  
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
color.lightness(); // 0.8039

color.hsv_hue(); // 210.0
color.hsv_saturation(); // 0.2845
color.hsv_value(); // 0.9373

color.luma(); // 0.59
color.luminance(); // 0.79

color.gray(); // 198.71
```

</details>
  
### Color Operation

- [darken](https://color-art.netlify.app/api/operation#darken) - Darken a color.
- [lighten](https://color-art.netlify.app/api/operation#lighten) - Lighten a color.
- [saturate](https://color-art.netlify.app/api/operation#saturate) - Saturate a color.
- [desaturate](https://color-art.netlify.app/api/operation#desaturate) - Desaturate a color.
- [greyscale](https://color-art.netlify.app/api/operation#greyscale) - Greyscale a color.
- [fade](https://color-art.netlify.app/api/operation#fade) - Fade a color.
- [fade in](https://color-art.netlify.app/api/operation#fade_in) - Fade in a color.
- [fade out](https://color-art.netlify.app/api/operation#fade_out) - Fade out a color.
- [spin](https://color-art.netlify.app/api/operation#spin) - Spin a color.
- [mix](https://color-art.netlify.app/api/operation#mix) - Mix with another color.
- [tint](https://color-art.netlify.app/api/operation#tint) - Tint a color.
- [shade](https://color-art.netlify.app/api/operation#shade) - Shade a color.

More details, please refer to [Color Operation](https://color-art.netlify.app/api/operation).

---


Made with ❤️ by [JiatLn](https://github.com/JiatLn).

## License

[MIT](./LICENSE) License © 2022-Present [JiatLn](https://github.com/JiatLn)
