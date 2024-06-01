/// A Pixel made up of a compile-time known number of contiguously stored `T`s.
///
/// Usually `T` is a small copiable intrinsic type such as `u8`, `u16` or `f32`.
pub trait Pixel {
    /// The component type of the pixel used for both color and alpha components if any.
    type Component;

    /// The total number of components in the pixel.
    ///
    /// If the pixel contains an alpha components then this number should be equal to the number of
    /// color components + 1. That is, you cannot have more than 1 alpha components, but you can
    /// have 0.
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
    fn map_colors(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;

    /// Map the pixel to the same outer pixel type with an optionally different inner color component
    /// type and the exact same alpha component.
    fn map_alpha(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self;
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

            fn from_components(components: Self::Array<Self::Component>) -> Self {
                let mut iter = components.into_iter();

                Self {$($bit: iter.next().unwrap()),*}
            }
            fn from_colors_alpha(colors: Self::ColorArray<Self::Component>, _: Self::Component) -> Self {
                let mut iter = colors.into_iter();

                Self {$($bit: iter.next().unwrap()),*}
            }

            fn map<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
            where
                U: Copy,
            {
                Self::SelfType::from_components(self.components().map(f))
            }

            fn map_colors(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                self.map(f)
            }

            fn map_alpha(&self, _: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                *self
            }
        }
    }
}

macro_rules! implement_pixel_with_alpha {
    ($name:tt, $length:literal, [$($bit:ident),*], [$($color_bit:ident),*], $alpha_bit:ident) => {
        impl<T> Pixel for $name<T>
        where
            T: Copy,
        {
            type Component = T;

            const COMPONENT_COUNT: u8 = $length;

            type SelfType<U> = $name<U>;
            type Array<R> = [R; $length];
            type ColorArray<R> = [R; $length - 1];

            fn components(&self) -> Self::Array<Self::Component> {
                [$(self.$bit),*]
            }
            fn colors(&self) -> Self::ColorArray<Self::Component> {
                [$(self.$color_bit),*]
            }

            fn from_components(components: Self::Array<Self::Component>) -> Self {
                let mut iter = components.into_iter();

                Self {$($bit: iter.next().unwrap()),*}
            }
            fn from_colors_alpha(colors: Self::ColorArray<Self::Component>, alpha: Self::Component) -> Self {
                let mut iter = colors.into_iter();

                Self {$($color_bit: iter.next().unwrap()),*, $alpha_bit: alpha}
            }

            fn map<U>(&self, f: impl FnMut(Self::Component) -> U) -> Self::SelfType<U>
            where
                U: Copy,
            {
                Self::SelfType::from_components(self.components().map(f))
            }

            fn map_colors(&self, f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                Self::SelfType::from_colors_alpha(self.colors().map(f), self.$alpha_bit)
            }

            fn map_alpha(&self, mut f: impl FnMut(Self::Component) -> Self::Component) -> Self
            {
                Self::SelfType::from_colors_alpha(self.colors(), f(self.$alpha_bit))
            }
        }
    }
}

mod rgba {
    use crate::*;
    implement_pixel_with_alpha!(Rgba, 4, [r, g, b, a], [r, g, b], a);
}
mod abgr {
    use crate::*;
    implement_pixel_with_alpha!(Abgr, 4, [a, b, g, r], [b, g, r], a);
}
mod argb {
    use crate::*;
    implement_pixel_with_alpha!(Argb, 4, [a, r, g, b], [r, g, b], a);
}
mod bgra {
    use crate::*;
    implement_pixel_with_alpha!(Bgra, 4, [b, g, r, a], [b, g, r], a);
}
mod gray_alpha {
    use crate::*;
    implement_pixel_with_alpha!(GrayAlpha, 2, [gray, a], [gray], a);
}

mod gray {
    use crate::*;
    implement_pixel_without_alpha!(Gray, 1, [gray]);
}
mod bgr {
    use crate::*;
    implement_pixel_without_alpha!(Bgr, 3, [b, g, r]);
}
mod grb {
    use crate::*;
    implement_pixel_without_alpha!(Grb, 3, [g, r, b]);
}
mod rgb {
    use crate::*;
    implement_pixel_without_alpha!(Rgb, 3, [r, g, b]);
}
