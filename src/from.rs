use crate::*;

macro_rules! implement_from {
    ($from_type:ident, $self_type:ident, {$($bit:ident),*}) => {
        impl<T> From<$from_type<T>> for $self_type<T> {
            fn from(value: $from_type<T>) -> Self {
                Self{$($bit: value.$bit),*}
            }
        }
    };
}

implement_from!(Rgba, Rgb, {r, g, b});
implement_from!(Argb, Rgb, {r, g, b});
implement_from!(Bgra, Rgb, {r, g, b});
implement_from!(Abgr, Rgb, {r, g, b});
implement_from!(Bgr, Rgb, {r, g, b});
implement_from!(Grb, Rgb, {r, g, b});

implement_from!(Rgba, Bgr, {r, g, b});
implement_from!(Argb, Bgr, {r, g, b});
implement_from!(Bgra, Bgr, {r, g, b});
implement_from!(Abgr, Bgr, {r, g, b});
implement_from!(Rgb, Bgr, {r, g, b});
implement_from!(Grb, Bgr, {r, g, b});

implement_from!(Rgba, Grb, {r, g, b});
implement_from!(Argb, Grb, {r, g, b});
implement_from!(Bgra, Grb, {r, g, b});
implement_from!(Abgr, Grb, {r, g, b});
implement_from!(Rgb, Grb, {r, g, b});
implement_from!(Bgr, Grb, {r, g, b});

implement_from!(Argb, Rgba, {r, g, b, a});
implement_from!(Bgra, Rgba, {r, g, b, a});
implement_from!(Abgr, Rgba, {r, g, b, a});

implement_from!(Rgba, Argb, {r, g, b, a});
implement_from!(Bgra, Argb, {r, g, b, a});
implement_from!(Abgr, Argb, {r, g, b, a});

implement_from!(Rgba, Bgra, {r, g, b, a});
implement_from!(Argb, Bgra, {r, g, b, a});
implement_from!(Abgr, Bgra, {r, g, b, a});

implement_from!(Rgba, Abgr, {r, g, b, a});
implement_from!(Argb, Abgr, {r, g, b, a});
implement_from!(Bgra, Abgr, {r, g, b, a});

implement_from!(GrayAlpha, Gray, { gray });
