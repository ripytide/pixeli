/// Convert between pixel component types using common component ranges.
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
macro_rules! integer_integer {
    ($int1:ident, $int2:ident) => {
        impl FromComponentCommon<$int1> for $int2 {
            fn from_component_common(component: $int1) -> Self {
                Self::from_component_common(f64::from_component_common(component))
            }
        }
        impl FromComponentCommon<$int2> for $int1 {
            fn from_component_common(component: $int2) -> Self {
                Self::from_component_common(f64::from_component_common(component))
            }
        }
    };
}

impl FromComponentCommon<f32> for f64 {
    fn from_component_common(component: f32) -> Self {
        component as f64
    }
}
impl FromComponentCommon<f64> for f32 {
    fn from_component_common(component: f64) -> Self {
        component as f32
    }
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
float_integer!(f32, isize);

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
float_integer!(f64, isize);

impl FromComponentCommon<u8> for u16 {
    fn from_component_common(component: u8) -> Self {
        let x = u16::from(component);
        (x << 8) | x
    }
}
impl FromComponentCommon<u16> for u8 {
    fn from_component_common(component: u16) -> Self {
        // The input c is the numerator of `c / u16::MAX`.
        // Derive numerator of `num / u8::MAX`, with rounding.
        //
        // This method is based on the inverse (see FromPrimitive<u8> for u16) and was tested
        // exhaustively in Python. It's the same as the reference function:
        //  round(c * (2**8 - 1) / (2**16 - 1))
        ((u32::from(component) + 128) / 257) as u8
    }
}
integer_integer!(u8, u32);
integer_integer!(u8, u64);
integer_integer!(u8, u128);
integer_integer!(u8, i8);
integer_integer!(u8, i16);
integer_integer!(u8, i32);
integer_integer!(u8, i64);
integer_integer!(u8, i128);
integer_integer!(u8, usize);
integer_integer!(u8, isize);

integer_integer!(u16, u32);
integer_integer!(u16, u64);
integer_integer!(u16, u128);
integer_integer!(u16, i8);
integer_integer!(u16, i16);
integer_integer!(u16, i32);
integer_integer!(u16, i64);
integer_integer!(u16, i128);
integer_integer!(u16, usize);
integer_integer!(u16, isize);

integer_integer!(u32, u64);
integer_integer!(u32, u128);
integer_integer!(u32, i8);
integer_integer!(u32, i16);
integer_integer!(u32, i32);
integer_integer!(u32, i64);
integer_integer!(u32, i128);
integer_integer!(u32, usize);
integer_integer!(u32, isize);

integer_integer!(u64, u128);
integer_integer!(u64, i8);
integer_integer!(u64, i16);
integer_integer!(u64, i32);
integer_integer!(u64, i64);
integer_integer!(u64, i128);
integer_integer!(u64, usize);
integer_integer!(u64, isize);

integer_integer!(u128, i8);
integer_integer!(u128, i16);
integer_integer!(u128, i32);
integer_integer!(u128, i64);
integer_integer!(u128, i128);
integer_integer!(u128, usize);
integer_integer!(u128, isize);

integer_integer!(i8, i16);
integer_integer!(i8, i32);
integer_integer!(i8, i64);
integer_integer!(i8, i128);
integer_integer!(i8, usize);
integer_integer!(i8, isize);

integer_integer!(i16, i32);
integer_integer!(i16, i64);
integer_integer!(i16, i128);
integer_integer!(i16, usize);
integer_integer!(i16, isize);

integer_integer!(i32, i64);
integer_integer!(i32, i128);
integer_integer!(i32, usize);
integer_integer!(i32, isize);

integer_integer!(i64, i128);
integer_integer!(i64, usize);
integer_integer!(i64, isize);

integer_integer!(i128, usize);
integer_integer!(i128, isize);

integer_integer!(usize, isize);
