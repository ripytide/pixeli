#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An `RGB` pixel.
///
/// The component type can be `u8` (aliased as [`RgbU8`]), `u16` (aliased as [`RgbU16`]),
/// or any other type (but simple copyable types are recommended).
pub struct Rgb<T> {
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
}
