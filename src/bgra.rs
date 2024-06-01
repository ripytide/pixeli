#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `BGRA` pixel.
///
/// The component type can be `u8` (aliased as [`BgraU8`]), `u16` (aliased as [`BgraU16`]),
/// or any other type (but simple copyable types are recommended).
///
/// You can specify a different type for alpha, but it's only for special cases
/// (e.g. if you use a newtype like `Bgra<LinearLight<u16>, u16>`).
pub struct Bgra<T, A = T> {
    /// Blue Component
    pub b: T,
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
    /// Alpha Component
    pub a: A,
}
