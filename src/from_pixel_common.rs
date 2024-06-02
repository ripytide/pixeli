use crate::*;

/// Convert between pixel types using the most common conversion method, this conversion can be
/// lossy, such as from [`Rgb`] to [`Gray`].
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
    P: Pixel + WithAlpha,
    Q: Pixel,
    Q::Component: FromComponentCommon<<P::WithAlpha as Pixel>::Component>,
    Q: From<<P::WithAlpha as Pixel>::SelfType<Q::Component>>,
{
    let with_alpha = pixel.with_alpha();
    let converted = with_alpha.map_components(Q::Component::from_component_common);
    Q::from(converted)
}

macro_rules! lossless {
    ($from:ident, $into:ident) => {
        impl<R, S> FromPixelCommon<$from<R>> for $into<S>
        where
            R: PixelComponent,
            S: PixelComponent + FromComponentCommon<R>,
        {
            fn from_pixel_common(pixel: $from<R>) -> Self {
                lossless(pixel)
            }
        }
    };
}
macro_rules! lossless_with_alpha {
    ($from:ident, $into:ident) => {
        impl<R, S> FromPixelCommon<$from<R>> for $into<S>
        where
            R: PixelComponent,
            S: PixelComponent + FromComponentCommon<R>,
        {
            fn from_pixel_common(pixel: $from<R>) -> Self {
                lossless_with_alpha(pixel)
            }
        }
    };
}

lossless!(Rgb, Rgb);
lossless!(Rgb, Bgr);
lossless!(Rgb, Grb);
lossless_with_alpha!(Rgb, Rgba);
lossless_with_alpha!(Rgb, Argb);
lossless_with_alpha!(Rgb, Bgra);
lossless_with_alpha!(Rgb, Abgr);

lossless!(Bgr, Rgb);
lossless!(Bgr, Bgr);
lossless!(Bgr, Grb);

lossless!(Grb, Rgb);
lossless!(Grb, Bgr);
lossless!(Grb, Grb);

lossless!(Rgba, Rgb);
lossless!(Rgba, Bgr);
lossless!(Rgba, Grb);
lossless!(Rgba, Rgba);
lossless!(Rgba, Argb);
lossless!(Rgba, Bgra);
lossless!(Rgba, Abgr);

lossless!(Argb, Rgb);
lossless!(Argb, Bgr);
lossless!(Argb, Grb);
lossless!(Argb, Rgba);
lossless!(Argb, Argb);
lossless!(Argb, Bgra);
lossless!(Argb, Abgr);

lossless!(Bgra, Rgb);
lossless!(Bgra, Bgr);
lossless!(Bgra, Grb);
lossless!(Bgra, Rgba);
lossless!(Bgra, Argb);
lossless!(Bgra, Bgra);
lossless!(Bgra, Abgr);

lossless!(Abgr, Rgb);
lossless!(Abgr, Bgr);
lossless!(Abgr, Grb);
lossless!(Abgr, Rgba);
lossless!(Abgr, Argb);
lossless!(Abgr, Bgra);
lossless!(Abgr, Abgr);
