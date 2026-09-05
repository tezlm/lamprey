use gpui::{Background, Fill, Hsla, Rgba};
use std::fmt;

/// an oklch color
#[derive(Clone, Copy, PartialEq, Default)]
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
    pub a: f32,
}

impl fmt::Debug for Oklch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "oklch({} {} {} / {})", self.l, self.c, self.h, self.a)
    }
}

/// convert Linear sRGB to Standard sRGB (gamma correction)
fn srgb_transfer(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

impl Oklch {
    /// Convert the OKLCH color to a standard [`gpui::Rgba`] color.
    pub fn to_rgba(&self) -> Rgba {
        let hue_rad = self.h.to_radians();
        let a_coeff = self.c * hue_rad.cos();
        let b_coeff = self.c * hue_rad.sin();

        // Oklab to LMS
        let l_ = self.l + 0.3963377774 * a_coeff + 0.2158037573 * b_coeff;
        let m_ = self.l - 0.1055613458 * a_coeff - 0.0638541728 * b_coeff;
        let s_ = self.l - 0.0894841775 * a_coeff - 1.2914855480 * b_coeff;

        // LMS non-linear to linear
        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        // LMS to Linear sRGB
        let r_linear = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        let g_linear = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        let b_linear = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        Rgba {
            r: srgb_transfer(r_linear),
            g: srgb_transfer(g_linear),
            b: srgb_transfer(b_linear),
            a: self.a,
        }
    }
}

impl From<Oklch> for Rgba {
    fn from(color: Oklch) -> Self {
        color.to_rgba()
    }
}

impl From<Oklch> for Hsla {
    fn from(color: Oklch) -> Self {
        Hsla::from(color.to_rgba())
    }
}

impl From<Oklch> for Background {
    fn from(color: Oklch) -> Self {
        Background::from(Rgba::from(color))
    }
}

impl From<Oklch> for Fill {
    fn from(color: Oklch) -> Self {
        Fill::from(Background::from(color))
    }
}

#[macro_export]
macro_rules! oklch {
    ($l:expr, $c:expr, $h:expr) => {
        $crate::oklch::Oklch {
            l: $l.into(),
            c: $c.into(),
            h: $h.into(),
            a: 1.0,
        }
    };
    ($l:expr, $c:expr, $h:expr, $a:expr) => {
        $crate::oklch::Oklch {
            l: $l.into(),
            c: $c.into(),
            h: $h.into(),
            a: $a.into(),
        }
    };
}
