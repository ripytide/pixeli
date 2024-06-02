#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![no_std]

#[cfg(feature = "libm")]
extern crate libm;

mod abgr;
mod argb;
mod bgr;
mod bgra;
mod gray;
mod gray_alpha;
mod rgb;
mod rgba;

mod from;
mod from_pixel_common;
mod pixel;
mod with_alpha;

pub use abgr::Abgr;
pub use argb::Argb;
pub use bgr::Bgr;
pub use bgra::Bgra;
pub use gray::Gray;
pub use gray_alpha::GrayAlpha;
pub use rgb::Rgb;
pub use rgba::Rgba;

pub use from_pixel_common::FromPixelCommon;
pub use pixel::{Pixel, PixelComponent};
pub use with_alpha::{WithAlpha, WithoutAlpha};
