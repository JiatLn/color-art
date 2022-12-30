# Color Operation

## darken

Decrease the lightness of a color in the HSL color space by an absolute amount.

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#ffff00").unwrap();
color.darken(0.2).unwrap();
color.hex(); // "#999900"
```

## lighten

Increase the lightness of a color in the HSL color space by an absolute amount.

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#ffff00").unwrap();
color.lighten(0.2).unwrap();
color.hex(); // "#ffff66"
```

## saturate

Increase the saturation of a color in the HSL color space by an absolute amount.

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
color.saturate(0.2).unwrap();
color.hsl(); // "hsl(60, 100%, 50%)"
```

## desaturate

Decrease the saturation of a color in the HSL color space by an absolute amount.

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
color.desaturate(0.2).unwrap();
color.hsl(); // "hsl(60, 60%, 50%)"
```

## greyscale

Remove all saturation from a color in the HSL color space.
# Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#80e619").unwrap();
color.greyscale().unwrap();
color.hex(); // "#808080"
```

## fade

Set the absolute opacity of a color.

### Parameters

* `amount` - The opacity to set. Must be between 0 and 1.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#ff0000").unwrap();
let faded_color = color.fade(0.5).unwrap();
faded_color.rgba(); // "rgba(255, 0, 0, 0.5)"
```

## fade_in

Decrease the transparency (or increase the opacity) of a color, making it more opaque.

#### Parameters

* `amount` - The amount to fade in. Must be between 0 and 1.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("rgba(255, 0, 0, 0.8)").unwrap();
let faded_color = color.fade_in(0.1).unwrap();
faded_color.rgba(); // "rgba(255, 0, 0, 0.9)"
```

## fade_out

Increase the transparency (or decrease the opacity) of a color, making it less opaque.

#### Parameters

* `amount` - The amount to fade out. Must be between 0 and 1.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("rgba(255, 0, 0, 0.8)").unwrap();
let faded_color = color.fade_out(0.2).unwrap();
faded_color.rgba(); // "rgba(255, 0, 0, 0.6)"
```

## spin

Rotate the hue angle of a color in either direction.

#### Parameters

* `angle` - The angle to rotate the hue by. Positive values rotate clockwise, negative values rotate counter-clockwise.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let color = Color::from_str("hsl(60, 80%, 50%)").unwrap();
let spun_color = color.spin(80.0).unwrap();
spun_color.hsl(); // "hsl(140, 80%, 50%)"
```

## mix_with

Mix a color with another color by a given amount.

#### Parameters

* `other` - The color to mix with.
* `amount` - The amount to mix the colors by. Must be between 0 and 1.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#ff0000").unwrap();
let other_color = Color::from_str("#0000ff").unwrap();
color.mix_with(&other_color, 0.5).unwrap()
color.hex(); // "#800080"
```

## tint

Tint a color by mixing it with white.

#### Parameters

* `amount` - The amount to tint the color by. Must be between 0 and 1.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#ff00ff").unwrap();
color.tint(0.5).unwrap();
color.hex(); // "#ff80ff"
```

## shade

Shade a color by mixing it with black.

#### Parameters

* `amount` - The amount to shade the color by. Must be between 0 and 1.

#### Example

```rust
use color_art::Color;
use std::str::FromStr;

let mut color = Color::from_str("#ff00ff").unwrap();
color.shade(0.5).unwrap();
color.hex(); // "#800080"
```
