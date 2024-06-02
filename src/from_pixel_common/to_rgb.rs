use crate::*;

/// ToRgb can also mean including alpha, for example from [`GrayAlpha`] -> [`Rgba`].
pub trait ToRgb: Pixel {
    type Rgb: Pixel;

    fn to_rgb(self) -> Self::Rgb;
}

impl<T> ToRgb for Gray<T>
where
    T: PixelComponent,
{
    type Rgb = Rgb<T>;

    fn to_rgb(self) -> Self::Rgb {
        Self::Rgb {
            r: self.gray,
            g: self.gray,
            b: self.gray,
        }
    }
}

impl<T> ToRgb for GrayAlpha<T>
where
    T: PixelComponent,
{
    type Rgb = Rgba<T>;

    fn to_rgb(self) -> Self::Rgb {
        Self::Rgb {
            r: self.gray,
            g: self.gray,
            b: self.gray,
            a: self.a,
        }
    }
}
