use crate::*;

/// Convert between pixel types using component-wise conversions using the [`FromComponentCommon`]
/// trait.
pub trait FromPixelCommon<P> {
    fn from_pixel_common(pixel: P) -> Self;
}

macro_rules! from_pixel_common {
    ($pixel1:ident, $pixel2:ident, $block:block) => {
        impl<S, T> FromPixelCommon<$pixel1<S>> for $pixel2<T>
        where
            T: PixelComponent + FromComponentCommon<S>,
            S: PixelComponent,
        {
            fn from_pixel_common(pixel: $pixel1<S>) -> Self $block
        }
    };
}
from_pixel_common!(Gray, Gray, {
    pixel.map_components(T::from_component_common)
});
impl<S, T> FromPixelCommon<GrayAlpha<S>> for Gray<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: GrayAlpha<S>) -> Self {
        self.channels_mut()[0] = T::from_primitive(other.channels()[0])
    }
}
impl<S, T> FromPixelCommon<Rgb<S>> for Gray<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgb<S>) -> Self {
        let gray = self.channels_mut();
        let rgb = other.channels();
        gray[0] = T::from_primitive(rgb_to_luma(rgb));
    }
}
impl<S, T> FromPixelCommon<Rgba<S>> for Gray<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgba<S>) -> Self {
        let gray = self.channels_mut();
        let rgb = other.channels();
        let l = rgb_to_luma(rgb);
        gray[0] = T::from_primitive(l);
    }
}
impl<S, T> FromPixelCommon<GrayAlpha<S>> for GrayAlpha<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: GrayAlpha<S>) -> Self {
        let own = self.channels_mut();
        let other = other.channels();
        own[0] = T::from_primitive(other[0]);
        own[1] = T::from_primitive(other[1]);
    }
}
impl<S, T> FromPixelCommon<Rgb<S>> for GrayAlpha<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgb<S>) -> Self {
        let gray_a = self.channels_mut();
        let rgb = other.channels();
        gray_a[0] = T::from_primitive(rgb_to_luma(rgb));
        gray_a[1] = T::DEFAULT_MAX_VALUE;
    }
}
impl<S, T> FromPixelCommon<Rgba<S>> for GrayAlpha<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgba<S>) -> Self {
        let gray_a = self.channels_mut();
        let rgba = other.channels();
        gray_a[0] = T::from_primitive(rgb_to_luma(rgba));
        gray_a[1] = T::from_primitive(rgba[3]);
    }
}
impl<S, T> FromPixelCommon<Gray<S>> for GrayAlpha<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Gray<S>) -> Self {
        let gray_a = self.channels_mut();
        gray_a[0] = T::from_primitive(other.channels()[0]);
        gray_a[1] = T::DEFAULT_MAX_VALUE;
    }
}
impl<S, T> FromPixelCommon<Rgba<S>> for Rgba<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgba<S>) -> Self {
        let own = &mut self.0;
        let other = &other.0;
        own[0] = T::from_primitive(other[0]);
        own[1] = T::from_primitive(other[1]);
        own[2] = T::from_primitive(other[2]);
        own[3] = T::from_primitive(other[3]);
    }
}
impl<S, T> FromPixelCommon<Rgb<S>> for Rgba<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgb<S>) -> Self {
        let rgba = &mut self.0;
        let rgb = &other.0;
        rgba[0] = T::from_primitive(rgb[0]);
        rgba[1] = T::from_primitive(rgb[1]);
        rgba[2] = T::from_primitive(rgb[2]);
        rgba[3] = T::DEFAULT_MAX_VALUE;
    }
}
impl<S, T> FromPixelCommon<GrayAlpha<S>> for Rgba<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(&mut self, gray: &GrayAlpha<S>) -> Self {
        let rgba = &mut self.0;
        let gray = &gray.0;
        rgba[0] = T::from_primitive(gray[0]);
        rgba[1] = T::from_primitive(gray[0]);
        rgba[2] = T::from_primitive(gray[0]);
        rgba[3] = T::from_primitive(gray[1]);
    }
}
impl<S, T> FromPixelCommon<Gray<S>> for Rgba<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(&mut self, gray: &Gray<S>) -> Self {
        let rgba = &mut self.0;
        let gray = gray.0[0];
        rgba[0] = T::from_primitive(gray);
        rgba[1] = T::from_primitive(gray);
        rgba[2] = T::from_primitive(gray);
        rgba[3] = T::DEFAULT_MAX_VALUE;
    }
}
impl<S, T> FromPixelCommon<Rgb<S>> for Rgb<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgb<S>) -> Self {
        let own = &mut self.0;
        let other = &other.0;
        own[0] = T::from_primitive(other[0]);
        own[1] = T::from_primitive(other[1]);
        own[2] = T::from_primitive(other[2]);
    }
}
impl<S, T> FromPixelCommon<Rgba<S>> for Rgb<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Rgba<S>) -> Self {
        let rgb = &mut self.0;
        let rgba = &other.0;
        rgb[0] = T::from_primitive(rgba[0]);
        rgb[1] = T::from_primitive(rgba[1]);
        rgb[2] = T::from_primitive(rgba[2]);
    }
}
impl<S, T> FromPixelCommon<GrayAlpha<S>> for Rgb<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: GrayAlpha<S>) -> Self {
        let rgb = &mut self.0;
        let gray = other.0[0];
        rgb[0] = T::from_primitive(gray);
        rgb[1] = T::from_primitive(gray);
        rgb[2] = T::from_primitive(gray);
    }
}
impl<S, T> FromPixelCommon<Gray<S>> for Rgb<T>
where
    T: FromComponentCommon<S>,
{
    fn from_pixel_common(pixel: Gray<S>) -> Self {
        let rgb = &mut self.0;
        let gray = other.0[0];
        rgb[0] = T::from_primitive(gray);
        rgb[1] = T::from_primitive(gray);
        rgb[2] = T::from_primitive(gray);
    }
}

/// Convert between pixel component types using common component ranges such as using the 0-1 range
/// for `f32`.
pub trait FromComponentCommon<T> {
    /// Converts to this type from the input component type.
    fn from_component_common(component: T) -> Self;
}

impl<T> FromComponentCommon<T> for T {
    fn from_component_common(component: T) -> Self {
        component
    }
}
#[cfg(feature = "libm")]
impl FromComponentCommon<f32> for u8 {
    fn from_component_common(float: f32) -> Self {
        libm::roundf(float.clamp(0.0, 1.0) * u8::MAX as f32) as u8
    }
}
#[cfg(feature = "libm")]
impl FromComponentCommon<f32> for u16 {
    fn from_component_common(float: f32) -> Self {
        libm::roundf(float.clamp(0.0, 1.0) * u16::MAX as f32) as u16
    }
}
impl FromComponentCommon<u16> for u8 {
    fn from_component_common(c16: u16) -> Self {
        // The input c is the numerator of `c / u16::MAX`.
        // Derive numerator of `num / u8::MAX`, with rounding.
        //
        // This method is based on the inverse (see FromComponentCommon<u8> for u16) and was tested
        // exhaustively in Python. It's the same as the reference function:
        //  round(c * (2**8 - 1) / (2**16 - 1))
        ((u32::from(c16) + 128) / 257) as u8
    }
}
impl FromComponentCommon<u16> for f32 {
    fn from_component_common(int: u16) -> Self {
        (int as f32 / u16::MAX as f32).clamp(0.0, 1.0)
    }
}
impl FromComponentCommon<u8> for f32 {
    fn from_component_common(int: u8) -> Self {
        (int as f32 / u8::MAX as f32).clamp(0.0, 1.0)
    }
}
impl FromComponentCommon<u8> for u16 {
    fn from_component_common(c8: u8) -> Self {
        let x = u64::from(c8);
        ((x << 8) | x) as u16
    }
}
