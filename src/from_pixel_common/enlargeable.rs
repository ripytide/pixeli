use num_traits::NumCast;

use crate::*;

/// An `Enlargable::Larger` value should be enough to calculate
/// the sum (average) of a few hundred or thousand Enlargeable values.
pub trait Enlargeable: PixelComponent {
    /// The larger type
    type Larger: PixelComponent;

    /// Clamps back into `Self` from `Self::Larger`
    fn clamp_from(n: Self::Larger) -> Self {
        if n > Self::COMPONENT_MAX.to_larger() {
            Self::COMPONENT_MAX
        } else if n < Self::COMPONENT_MIN.to_larger() {
            Self::COMPONENT_MIN
        } else {
            NumCast::from(n).unwrap()
        }
    }

    /// Converts `self` to the `Self::Larger` type
    fn to_larger(self) -> Self::Larger {
        NumCast::from(self).unwrap()
    }
}

macro_rules! implement_enlargeable {
    ($base:ident, $larger:ident) => {
        impl Enlargeable for $base {
            type Larger = $larger;
        }
    };
}

//u16 is not large enough for the ToGray conversion so we use u32 instead
implement_enlargeable!(u8, u32);
implement_enlargeable!(u16, u32);
implement_enlargeable!(u32, u64);
implement_enlargeable!(u64, u128);
// Note: On 32-bit architectures, u64 should be enough here.
implement_enlargeable!(usize, u128);

//i16 is not large enough for the ToGray conversion so we use i32 instead
implement_enlargeable!(i8, i32);
implement_enlargeable!(i16, i32);
implement_enlargeable!(i32, i64);
implement_enlargeable!(i64, i128);
// Note: On 32-bit architectures, i64 should be enough here.
implement_enlargeable!(isize, i128);

implement_enlargeable!(f32, f64);
implement_enlargeable!(f64, f64);
