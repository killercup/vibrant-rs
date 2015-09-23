#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate image;
extern crate color_quant;
extern crate itertools;

mod settings;
mod hsl;
mod palette;
mod vibrant;

pub use vibrant::Vibrancy;
pub use palette::Palette;
