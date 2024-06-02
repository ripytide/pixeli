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
        libm::roundf(float.clamp(0.0, 1.0) * f32::from(u8::MAX)) as u8
    }
}
#[cfg(feature = "libm")]
impl FromComponentCommon<f32> for u16 {
    fn from_component_common(float: f32) -> Self {
        libm::roundf(float.clamp(0.0, 1.0) * f32::from(u16::MAX)) as u16
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
        (f32::from(int) / f32::from(u16::MAX)).clamp(0.0, 1.0)
    }
}
impl FromComponentCommon<u8> for f32 {
    fn from_component_common(int: u8) -> Self {
        (f32::from(int) / f32::from(u8::MAX)).clamp(0.0, 1.0)
    }
}
impl FromComponentCommon<u8> for u16 {
    fn from_component_common(c8: u8) -> Self {
        let x = u64::from(c8);
        ((x << 8) | x) as u16
    }
}
