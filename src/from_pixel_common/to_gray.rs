use num_traits::NumCast;

use crate::*;

use super::enlargeable::Enlargeable;

pub trait ToGray: Pixel {
    type Gray: Pixel;

    fn to_gray(self) -> Self::Gray;
}

macro_rules! implement_to_gray_without_alpha {
    ($from:ident) => {
        impl<T> ToGray for $from<T>
        where
            T: Enlargeable,
        {
            type Gray = Gray<T>;

            fn to_gray(self) -> Self::Gray {
                /// Coefficients to transform from sRGB to a CIE Y (luminance) value.
                const SRGB_LUMA: [u16; 3] = [2126, 7152, 722];
                const SRGB_LUMA_DIV: u16 = 10000;

                let l = <T::Larger as NumCast>::from(SRGB_LUMA[0]).unwrap() * self.r.to_larger()
                    + <T::Larger as NumCast>::from(SRGB_LUMA[1]).unwrap() * self.g.to_larger()
                    + <T::Larger as NumCast>::from(SRGB_LUMA[2]).unwrap() * self.b.to_larger();

                Gray {
                    gray: T::clamp_from(l / <T::Larger as NumCast>::from(SRGB_LUMA_DIV).unwrap()),
                }
            }
        }
    };
}
macro_rules! implement_to_gray_with_alpha {
    ($from:ident) => {
        impl<T> ToGray for $from<T>
        where
            T: Enlargeable,
        {
            type Gray = GrayAlpha<T>;

            fn to_gray(self) -> Self::Gray {
                /// Coefficients to transform from sRGB to a CIE Y (luminance) value.
                const SRGB_LUMA: [u16; 3] = [2126, 7152, 722];
                const SRGB_LUMA_DIV: u16 = 10000;

                let l = <T::Larger as NumCast>::from(SRGB_LUMA[0]).unwrap() * self.r.to_larger()
                    + <T::Larger as NumCast>::from(SRGB_LUMA[1]).unwrap() * self.g.to_larger()
                    + <T::Larger as NumCast>::from(SRGB_LUMA[2]).unwrap() * self.b.to_larger();

                GrayAlpha {
                    gray: T::clamp_from(l / <T::Larger as NumCast>::from(SRGB_LUMA_DIV).unwrap()),
                    a: self.a,
                }
            }
        }
    };
}

implement_to_gray_without_alpha!(Rgb);
implement_to_gray_without_alpha!(Bgr);
implement_to_gray_with_alpha!(Rgba);
implement_to_gray_with_alpha!(Argb);
implement_to_gray_with_alpha!(Bgra);
implement_to_gray_with_alpha!(Abgr);
