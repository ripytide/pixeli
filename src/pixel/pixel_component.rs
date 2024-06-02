use num_traits::{Num, NumCast};

/// A trait for all the required super-traits for a pixel component type.
///
/// It has a blanket implementation so you just need to make sure your type implements the
/// super-traits.
pub trait PixelComponent: Copy + Num + NumCast + PartialOrd<Self> + BoundedComponent {}
impl<T> PixelComponent for T where T: Copy + Num + NumCast + PartialOrd<Self> + BoundedComponent {}

pub trait BoundedComponent {
    const COMPONENT_MIN: Self;
    const COMPONENT_MAX: Self;
}
macro_rules! implement_bounded_integer {
    ($int:ident) => {
        impl BoundedComponent for $int {
            const COMPONENT_MIN: Self = $int::MIN;
            const COMPONENT_MAX: Self = $int::MAX;
        }
    };
}
macro_rules! implement_bounded_float {
    ($int:ident) => {
        impl BoundedComponent for $int {
            const COMPONENT_MIN: Self = 0.0;
            const COMPONENT_MAX: Self = 1.0;
        }
    };
}
implement_bounded_integer!(u8);
implement_bounded_integer!(u16);
implement_bounded_integer!(u32);
implement_bounded_integer!(u64);
implement_bounded_integer!(u128);
implement_bounded_integer!(i8);
implement_bounded_integer!(i16);
implement_bounded_integer!(i32);
implement_bounded_integer!(i64);
implement_bounded_integer!(i128);
implement_bounded_integer!(usize);
implement_bounded_integer!(isize);
implement_bounded_float!(f32);
implement_bounded_float!(f64);
