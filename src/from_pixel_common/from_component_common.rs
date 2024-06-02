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

macro_rules! float_integer {
    ($float:ident, $from:ident) => {
        impl FromComponentCommon<$from> for $float {
            fn from_component_common(component: $from) -> Self {
                ((component as $float - $from::MIN as $float)
                    / ($from::MAX as $float - $from::MIN as $float))
                    .clamp(0.0, 1.0)
            }
        }
        #[cfg(feature = "libm")]
        impl FromComponentCommon<$float> for $from {
            fn from_component_common(component: $float) -> Self {
                #[allow(unused_imports)]
                use num_traits::Float;

                (component.clamp(0.0, 1.0)
                    * $float::from($from::MAX as $float - $from::MIN as $float))
                .round() as $from
            }
        }
    };
}

float_integer!(f32, u8);
float_integer!(f32, u16);
float_integer!(f32, u32);
float_integer!(f32, u64);
float_integer!(f32, u128);
float_integer!(f32, i8);
float_integer!(f32, i16);
float_integer!(f32, i32);
float_integer!(f32, i64);
float_integer!(f32, i128);
float_integer!(f32, usize);

float_integer!(f64, u8);
float_integer!(f64, u16);
float_integer!(f64, u32);
float_integer!(f64, u64);
float_integer!(f64, u128);
float_integer!(f64, i8);
float_integer!(f64, i16);
float_integer!(f64, i32);
float_integer!(f64, i64);
float_integer!(f64, i128);
float_integer!(f64, usize);

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
impl FromComponentCommon<u8> for u16 {
    fn from_component_common(c8: u8) -> Self {
        let x = u64::from(c8);
        ((x << 8) | x) as u16
    }
}
