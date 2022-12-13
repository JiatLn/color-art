# Construct Color from Color Space

- Feature Name: `Construct Color from Color Space`
- Start Date: 2022-12-13 20:37:49

## Summary

Color can be constructed from color space. The color space can be any of the following: `RGB`, `RGBA`, `HSL`, `HSV`, `Lab`, `Hex`, `Named`, `Num` and so on. We can construct a color from any of these color spaces with `&str` by using the `from` method.

## Motivation

The motivation for this feature is to make it easier to construct a color from a color space. This is useful when you want to construct a color from a color space, and use the color in a function that takes a `Color` as an argument.

## Guide-level explanation

We need a struct to store the color. We can construct a color from a string that represents a color with color space and store the color in the struct.

The color space can be any of the following: `RGB`, `RGBA`, `HSL`, `HSV`, `Lab`, `Hex`, `Named`, `Num` and so on. We can construct a color from any of these color spaces with `&str` by using the `from_str` method.

### Struct

- Color

We base the RGB color space on the `Color` struct. The `Color` struct has the following fields:

```rust
struct Color {
  rgba: (f64, f64, f64, f64),
}
```

We store the color in the `RGBA` color space. And provide methods to convert the color to other color spaces.

For precision, we use `f64` to store the color. Rhe rgba field is a tuple of four `f64` values. The first three values are the red, green and blue values, which ranges from `0..255`. The last value is the alpha value, which ranges from `0..1`.

### `from_str` method

The `from_str` method will take a `&str` as an argument. The `&str` will be parsed and the color will be constructed from the color space. For example, if the `&str` is `"red"`, then the color will be constructed from the `Named` color space.

So, we need a method to parse the `&str` and construct the color from the color space. We can use the `FromStr` trait to do this. The `FromStr` trait has the following method:

```rust
trait FromStr {
  type Err;

  fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

Implementing the `FromStr` trait for the `Color` struct will allow us to use the `from_str` method on the `Color` struct.

```rust
impl FromStr for Color {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Parse the string and construct the color from the color space.
  }
}
```

After implementing the `FromStr` trait, we can use the `from_str` method on the `Color` struct.

```rust
let color = Color::from_str("red");
```


## Reference-level explanation

Here are some things that we need to consider:

1. We should consider which color spaces that can be passed to the `from_str` method.

2. We should also consider the precision of the color. We should use `f64` to store the color.

3. We should also consider the error handling. We should return an error if the `&str` cannot be parsed.

### Supported color spaces

Here is a list of all the color spaces that can be passed to the `from_str` method:

- `RGB`
- `RGBA`
- `HSL`
- `HSV`
- `Lab`
- `Hex`
- `Named`
- `Num`

### Precision

We use `f64` to store the color. The `rgba` field is a tuple of four `f64` values. The first three values are the red, green and blue values, which ranges from `0..255`. The last value is the alpha value, which ranges from `0..1`.

And diffrent color spaces will be converted to the `RGBA` color space. For example, if the color space is `HSL`, then the color will be converted to the `RGBA` color space. It will had precision loss.

### Error handling

We should return an error if the `&str` cannot be parsed. We can use the `Result` type to handle the error.

We will use the crate `anyhow` to handle the error. The `anyhow` crate has the following method:

```rust
pub fn anyhow<E>(error: E) -> anyhow::Error
where
  E: Into<anyhow::Error>,
```

## Drawbacks

TBD

## Rationale and alternatives

TBD

## Prior art

Some prior art that inspired this project:

- [chroma.js](https://github.com/gka/chroma.js)

## Unresolved questions

TBD

## Future possibilities

TBD
