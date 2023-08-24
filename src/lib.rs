//! [![github]](https://github.com/JiatLn/color-art)&ensp;[![crates-io]](https://crates.io/crates/color-art)&ensp;[![docs-rs]](https://docs.rs/color-art)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! A rust crate for working with colors and color spaces.

#[macro_use]
extern crate lazy_static;

mod color;
mod color_calc;
mod color_generator;
mod color_ops;
mod color_space;
mod conversion;
mod data;
mod parser;
mod utils;

pub use color::Color;
pub use color_calc::blend::*;
pub use color_calc::distance::*;
pub use color_calc::delta_e::*;
pub use color_space::ColorSpace;
use data::alpha_hex_map::ALPHA_HEX_MAP;
use data::chinese_color::CHINESE_COLOR_HASHMAP as CHINESE_COLOR;
use data::w3cx11::W3CX11_HASHMAP as W3CX11;
