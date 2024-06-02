use crate::*;

/// Convert between pixel types using the most common conversion method, this conversion can be
/// lossy, such as from [`Rgb`] to [`Gray`].
pub trait FromPixelCommon<P> {
    /// Converts the given pixel type to the `Self` type.
    fn from_pixel_common(pixel: P) -> Self;
}

/// A blanket implementation for lossless pixel conversions using the possibly lossy
/// [`FromComponentCommon`] component conversion.
fn lossless_convert_common<P, Q>(pixel: P) -> Q
where
    P: Pixel,
    Q: Pixel,
    Q::Component: FromComponentCommon<P::Component>,
    Q: From<P::SelfType<Q::Component>>,
{
    let converted = pixel.map_components(Q::Component::from_component_common);
    Q::from(converted)
}
