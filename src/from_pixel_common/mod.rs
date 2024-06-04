use crate::*;

use self::{
    enlargeable::Enlargeable, from_component_common::FromComponentCommon, to_gray::ToGray,
    to_rgb::ToRgb,
};

pub mod enlargeable;
pub mod from_component_common;
mod to_gray;
mod to_rgb;

/// Convert between pixel types using the most common conversion method, this conversion can be
/// lossy, such as from [`Rgb`] to [`Gray`].
///
/// - Floats are considered to range between `0.0..=1.0`
/// - Integers are considered to range between `MIN..=MAX`
/// - If gaining an alpha component it set to the maximum value of the above range
/// - If retaining an alpha component it stays at the same value
///
/// This trait can convert from any of the pixel types in this crate with a primitive generic
/// component to any other pixel type in this crate with any other primitive generic component type.
///
/// # Examples
/// ```
/// use pixeli::*;
///
/// let _: Rgb<i32> = Rgb::from_pixel_common(Gray::<u8> { gray: 100 });
/// let _: Rgba<f32> = Rgba::from_pixel_common(Gray::<i8> { gray: 100 });
/// let _: GrayAlpha<u8> = GrayAlpha::from_pixel_common(Bgr::<f32> { b: 0.3, g: 0.6, r: 0.9 });
/// let _: Gray<i8> = Gray::from_pixel_common(Rgba::<f64> { r: 0.4, g: 0.7, b: 0.7, a: 0.5 });
/// ```
pub trait FromPixelCommon<P> {
    /// Converts the given pixel type to the `Self` type.
    fn from_pixel_common(pixel: P) -> Self;
}

fn lossless<P, Q>(pixel: P) -> Q
where
    P: Pixel,
    Q: Pixel,
    Q::Component: FromComponentCommon<P::Component>,
    Q: From<P::SelfType<Q::Component>>,
{
    let converted = pixel.map_components(Q::Component::from_component_common);
    Q::from(converted)
}
fn lossless_with_alpha<P, Q>(pixel: P) -> Q
where
    P: Pixel,
    Q: Pixel,
    Q::Component: FromComponentCommon<P::Component>,
    P::SelfType<Q::Component>: WithAlpha,
    Q: From<<P::SelfType<Q::Component> as WithAlpha>::WithAlpha>,
{
    let converted = pixel.map_components(Q::Component::from_component_common);
    let with_alpha = converted.with_alpha();
    Q::from(with_alpha)
}
fn to_gray<P, Q>(pixel: P) -> Q
where
    P: Pixel + ToGray,
    Q: Pixel,
    Q::Component: FromComponentCommon<<P::Gray as Pixel>::Component>,
    Q: From<<P::Gray as Pixel>::SelfType<Q::Component>>,
{
    let gray = pixel.to_gray();
    let converted = gray.map_components(Q::Component::from_component_common);
    Q::from(converted)
}
fn to_gray_with_alpha<P, Q>(pixel: P) -> Q
where
    P: Pixel + ToGray,
    Q: Pixel,
    Q::Component: FromComponentCommon<<P::Gray as Pixel>::Component>,
    <P::Gray as Pixel>::SelfType<Q::Component>: WithAlpha,
    Q: From<<<P::Gray as Pixel>::SelfType<Q::Component> as WithAlpha>::WithAlpha>,
{
    let gray = pixel.to_gray();
    let converted = gray.map_components(Q::Component::from_component_common);
    let with_alpha = converted.with_alpha();
    Q::from(with_alpha)
}
fn to_rgb<P, Q>(pixel: P) -> Q
where
    P: Pixel + ToRgb,
    Q: Pixel,
    Q::Component: FromComponentCommon<<P::Rgb as Pixel>::Component>,
    Q: From<<P::Rgb as Pixel>::SelfType<Q::Component>>,
{
    let gray = pixel.to_rgb();
    let converted = gray.map_components(Q::Component::from_component_common);
    Q::from(converted)
}
fn to_rgb_with_alpha<P, Q>(pixel: P) -> Q
where
    P: Pixel + ToRgb,
    Q: Pixel,
    Q::Component: FromComponentCommon<<P::Rgb as Pixel>::Component>,
    <P::Rgb as Pixel>::SelfType<Q::Component>: WithAlpha,
    Q: From<<<P::Rgb as Pixel>::SelfType<Q::Component> as WithAlpha>::WithAlpha>,
{
    let gray = pixel.to_rgb();
    let converted = gray.map_components(Q::Component::from_component_common);
    let with_alpha = converted.with_alpha();
    Q::from(with_alpha)
}

macro_rules! from_pixel_common {
    ($from:ident, $into:ident, $method:ident) => {
        impl<R, S> FromPixelCommon<$from<R>> for $into<S>
        where
            R: PixelComponent + Enlargeable,
            S: PixelComponent + FromComponentCommon<R>,
        {
            fn from_pixel_common(pixel: $from<R>) -> Self {
                $method(pixel)
            }
        }
    };
}

macro_rules! lossless {
    ($from:ident, $into:ident) => {
        from_pixel_common!($from, $into, lossless);
    };
}
macro_rules! lossless_with_alpha {
    ($from:ident, $into:ident) => {
        from_pixel_common!($from, $into, lossless_with_alpha);
    };
}
macro_rules! to_gray {
    ($from:ident, $into:ident) => {
        from_pixel_common!($from, $into, to_gray);
    };
}
macro_rules! to_gray_with_alpha {
    ($from:ident, $into:ident) => {
        from_pixel_common!($from, $into, to_gray_with_alpha);
    };
}
macro_rules! to_rgb {
    ($from:ident, $into:ident) => {
        from_pixel_common!($from, $into, to_rgb);
    };
}
macro_rules! to_rgb_with_alpha {
    ($from:ident, $into:ident) => {
        from_pixel_common!($from, $into, to_rgb_with_alpha);
    };
}

lossless!(Rgb, Rgb);
lossless!(Rgb, Bgr);
lossless_with_alpha!(Rgb, Rgba);
lossless_with_alpha!(Rgb, Argb);
lossless_with_alpha!(Rgb, Bgra);
lossless_with_alpha!(Rgb, Abgr);
to_gray!(Rgb, Gray);
to_gray_with_alpha!(Rgb, GrayAlpha);

lossless!(Bgr, Rgb);
lossless!(Bgr, Bgr);
lossless_with_alpha!(Bgr, Rgba);
lossless_with_alpha!(Bgr, Argb);
lossless_with_alpha!(Bgr, Bgra);
lossless_with_alpha!(Bgr, Abgr);
to_gray!(Bgr, Gray);
to_gray_with_alpha!(Bgr, GrayAlpha);

lossless!(Rgba, Rgb);
lossless!(Rgba, Bgr);
lossless!(Rgba, Rgba);
lossless!(Rgba, Argb);
lossless!(Rgba, Bgra);
lossless!(Rgba, Abgr);
to_gray!(Rgba, Gray);
to_gray!(Rgba, GrayAlpha);

lossless!(Argb, Rgb);
lossless!(Argb, Bgr);
lossless!(Argb, Rgba);
lossless!(Argb, Argb);
lossless!(Argb, Bgra);
lossless!(Argb, Abgr);
to_gray!(Argb, Gray);
to_gray!(Argb, GrayAlpha);

lossless!(Bgra, Rgb);
lossless!(Bgra, Bgr);
lossless!(Bgra, Rgba);
lossless!(Bgra, Argb);
lossless!(Bgra, Bgra);
lossless!(Bgra, Abgr);
to_gray!(Bgra, Gray);
to_gray!(Bgra, GrayAlpha);

lossless!(Abgr, Rgb);
lossless!(Abgr, Bgr);
lossless!(Abgr, Rgba);
lossless!(Abgr, Argb);
lossless!(Abgr, Bgra);
lossless!(Abgr, Abgr);
to_gray!(Abgr, Gray);
to_gray!(Abgr, GrayAlpha);

to_rgb!(Gray, Rgb);
to_rgb!(Gray, Bgr);
to_rgb_with_alpha!(Gray, Rgba);
to_rgb_with_alpha!(Gray, Argb);
to_rgb_with_alpha!(Gray, Bgra);
to_rgb_with_alpha!(Gray, Abgr);
lossless!(Gray, Gray);
lossless_with_alpha!(Gray, GrayAlpha);

to_rgb!(GrayAlpha, Rgb);
to_rgb!(GrayAlpha, Bgr);
to_rgb!(GrayAlpha, Rgba);
to_rgb!(GrayAlpha, Argb);
to_rgb!(GrayAlpha, Bgra);
to_rgb!(GrayAlpha, Abgr);
lossless!(GrayAlpha, Gray);
lossless!(GrayAlpha, GrayAlpha);
