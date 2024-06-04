/// A helpful trait for turning the associated array types in the [`Pixel`](crate::Pixel) trait
/// into a slice.
pub trait AsSlice<T> {
    /// Returns as a slice of `T`
    fn as_slice(&self) -> &[T];
}
impl<T, const N: usize> AsSlice<T> for [T; N] {
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
}
