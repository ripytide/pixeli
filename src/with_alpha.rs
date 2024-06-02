use crate::*;

trait WithAlpha: Pixel {
    type WithAlpha;

    fn with_alpha(pixel: Self) -> Self::WithAlpha;
}
trait WithoutAlpha: Pixel {
    type WithoutAlpha;

    fn without_alpha(pixel: Self) -> Self::WithoutAlpha;
}

macro_rules! implement_lower_upper {
    ($lower:ident, $upper:ident, {$($bit:ident),*}) => {
        impl<T> WithAlpha for $lower<T> where T: PixelComponent {
            type WithAlpha = $upper<T>;

            fn with_alpha(pixel: Self) -> Self::WithAlpha {
                $upper {
                    $($bit: pixel.$bit),*,
                    a: <$lower<T> as Pixel>::Component::max_value(),
                }
            }
        }
        impl<T> WithoutAlpha for $upper<T> where T: PixelComponent {
            type WithoutAlpha = $lower<T>;

            fn without_alpha(pixel: Self) -> Self::WithoutAlpha {
                $lower {
                    $($bit: pixel.$bit),*,
                }
            }
        }
    };
}
macro_rules! implement_with_no_op {
    ($original:ident) => {
        impl<T> WithAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type WithAlpha = $original<T>;

            fn with_alpha(pixel: Self) -> Self::WithAlpha {
                pixel
            }
        }
    };
}
macro_rules! implement_without_no_op {
    ($original:ident) => {
        impl<T> WithoutAlpha for $original<T>
        where
            T: PixelComponent,
        {
            type WithoutAlpha = $original<T>;

            fn without_alpha(pixel: Self) -> Self::WithoutAlpha {
                pixel
            }
        }
    };
}

implement_without_no_op!(Rgb);
implement_without_no_op!(Bgr);
implement_without_no_op!(Grb);
implement_without_no_op!(Gray);

implement_with_no_op!(Rgba);
implement_with_no_op!(Argb);
implement_with_no_op!(Bgra);
implement_with_no_op!(Abgr);
implement_with_no_op!(GrayAlpha);

implement_lower_upper!(Rgb, Rgba, {r, g, b});
implement_lower_upper!(Bgr, Bgra, {r, g, b});
implement_lower_upper!(Gray, GrayAlpha, { gray });
