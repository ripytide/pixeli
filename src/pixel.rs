/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait Pixel {
    type Component;

    const COMPONENT_COUNT: u8;

    /// The same pixel type as `Self` but with a different generic component type.
    type SelfType<U>;
    /// The array form of `Self`
    type Array<R>;
    /// The color array form of `Self`
    type ColorArray<R>;

    /// Converts an owned `Pixel` type to an array of its components.
    fn components(&self) -> Self::Array<Self::Component>;
    /// Converts an owned `Pixel` type to an array of its color components.
    fn colors(&self) -> Self::ColorArray<Self::Component>;

    /// Converts an array of components to a `Pixel`.
    fn from_components(components: Self::Array<Self::Component>) -> Self;
    /// Create a new instance given an array of its color components and an alpha component.
    fn from_colors_alpha(colors: Self::ColorArray<Self::Component>, alpha: Self::Component)
        -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner type.
    fn map<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
    where
        U: Copy;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_colors<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
    where
        U: Copy;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_alpha<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
    where
        U: Copy;
}

/// A Pixel with possibly differently typed color and alpha components.
pub trait HeterogeneousPixel<T, A> {
    /// The same pixel type as `Self` but with a different generic component types.
    type SelfType<U, B>;
    /// The color array form of `Self`
    type ColorArray<R>;
}

macro_rules! implement_pixel_without_alpha {
    ($name:ident, $length:literal, [$($bit:ident),*]) => {
        impl<T> Pixel for $name<T>
        where
            T: Copy,
        {
            type Component = T;

            const COMPONENT_COUNT: u8 = $length;

            type SelfType<U> = $name<U>;
            type Array<R> = [R; $length];
            type ColorArray<R> = [R; $length];

            fn components(&self) -> Self::Array<Self::Component> {
                [$(self.$bit),*]
            }
            fn colors(&self) -> Self::ColorArray<Self::Component> {
                [$(self.$bit),*]
            }

            fn from_components(components: Self::Array<T>) -> Self {
                let mut iter = components.into_iter();

                Self {$($bit: iter.next().unwrap()),*}
            }

            fn map<U>(&self, f: impl FnMut(T) -> U) -> Self::SelfType<U>
            where
                U: Copy,
            {
                Self::SelfType::from_components(self.components().map(f))
            }
        }
    }
}

// mod rgba {
//     use crate::*;
//     implement_pixel_without_alpha!(Rgba, 4, [r, g, b, a]);
//     implement_heterogeneous_pixel!(Rgba, 3, [r, g, b], a);
// }
// mod abgr {
//     use crate::*;
//     implement_pixel_without_alpha!(Abgr, 4, [a, b, g, r]);
//     implement_heterogeneous_pixel!(Abgr, 3, [b, g, r], a);
// }
// mod argb {
//     use crate::*;
//     implement_pixel_without_alpha!(Argb, 4, [a, r, g, b]);
//     implement_heterogeneous_pixel!(Argb, 3, [r, g, b], a);
// }
// mod bgra {
//     use crate::*;
//     implement_pixel_without_alpha!(Bgra, 4, [b, g, r, a]);
//     implement_heterogeneous_pixel!(Bgra, 3, [b, g, r], a);
// }
// mod gray_alpha {
//     use crate::*;
//     implement_pixel_without_alpha!(GrayAlpha, 2, [gray, a]);
//     implement_heterogeneous_pixel!(GrayAlpha, 1, [gray], a);
// }
//
// mod gray {
//     use crate::*;
//     implement_pixel_without_alpha!(Gray, 1, [gray]);
// }
// mod bgr {
//     use crate::*;
//     implement_pixel_without_alpha!(Bgr, 3, [b, g, r]);
// }
// mod grb {
//     use crate::*;
//     implement_pixel_without_alpha!(Grb, 3, [g, r, b]);
// }
mod rgb {
    use crate::*;
    implement_pixel_without_alpha!(Rgb, 3, [r, g, b]);
}
