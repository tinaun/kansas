//! 8 bit rgb and rgba color
//!
//! all operations on a `Canvas` use 8 bit color, with optional alpha channel

use std::mem;
use std::ops::Add;

// color pallete inspired by http://alumni.media.mit.edu/~wad/color/palette.html
/// <span style="border: 1px solid black; padding: 0 7px; background: #000000;">&nbsp;</span>
pub const BLACK: Rgb = Rgb(0.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: #FFFFFF;">&nbsp;</span>
pub const WHITE: Rgb = Rgb(1.0, 1.0, 1.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(87, 87, 87);">&nbsp;</span>
pub const DARK_GREY: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(160, 160, 160);">&nbsp;</span>
pub const LIGHT_GREY: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(205, 55, 55);">&nbsp;</span>
pub const RED: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(255, 205, 243);">&nbsp;</span>
pub const PINK: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(129, 38, 192);">&nbsp;</span>
pub const PURPLE: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(42, 75, 215);">&nbsp;</span>
pub const BLUE: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(157, 175, 255);">&nbsp;</span>
pub const LIGHT_BLUE: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(41, 208, 208);">&nbsp;</span>
pub const CYAN: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(29, 105, 20);">&nbsp;</span>
pub const GREEN: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(129, 197, 122);">&nbsp;</span>
pub const LIGHT_GREEN: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(255, 238, 51);">&nbsp;</span>
pub const YELLOW: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(255, 146, 51);">&nbsp;</span>
pub const ORANGE: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(129, 74, 25);">&nbsp;</span>
pub const BROWN: Rgb = Rgb(1.0, 0.0, 0.0);
/// <span style="border: 1px solid black; padding: 0 7px; background: rgb(233, 222, 187);">&nbsp;</span>
pub const LIGHT_BROWN: Rgb = Rgb(1.0, 0.0, 0.0);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
/// internal representation of a solid color value
///
pub struct Rgb(f32, f32, f32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
/// color value with alpha
pub struct Rgba(Rgb, f32);

impl Rgb {
    /// compute alpha
    pub fn blend_alpha(self, a: f32) -> Self {
        Rgb(self.0 * a, self.1 * a, self.2 * a)
    }

    /// mix two colors together with a certain weight
    ///
    /// this is a linear mix. `0xFF0000` mixed with `0x00FF00` at a weight of
    /// `0.5` is `0x808000`.
    pub fn mix(self, other: Self, weight: f32) -> Self {
        let a = self.blend_alpha(weight);
        let b = other.blend_alpha(1.0 - weight);

        a + b
    }

    fn clamp(self) -> Self {
        Rgb {
            0: self.0.max(0.0).min(1.0),
            1: self.1.max(0.0).min(1.0),
            2: self.2.max(0.0).min(1.0),
        }
    }
}

impl Add for Rgb {
    type Output = Rgb;
    fn add(self, rhs: Self) -> Self::Output {
        Rgb(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2).clamp()
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Rgb(0.0, 0.0, 0.0)
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Rgba(Default::default(), 1.0)
    }
}

impl From<(Rgb, f32)> for Rgba {
    fn from(tup: (Rgb, f32)) -> Self {
        Rgba(tup.0, tup.1)
    }
}

/// any value that could be used as a color
/// 
/// including:
///
///  * `(r, g, b)` triples and `(r, g, b, a)` 4-tuples for all floats and
/// (unsigned) int types
///  * `[r, g, b]`, and `[r, g, b, a]` arrays for floats and unsigned ints
///  * `0xRRGGBB` "color literals" for signed + unsigned ints
///  * constants defined in the module, ex. `color::RED` 
pub trait CanvasColor: Sized {
    fn as_rgb(&self) -> Rgb;
    fn alpha(&self) -> f32;
    fn into_gpu(self, prev: Option<Rgb>) -> [u8; 4] {
        let rgb = self.as_rgb();
        let alpha = self.alpha();
        
        let color = if let Some(color) = prev {
            rgb.mix(color, alpha)
        } else {
            rgb.blend_alpha(alpha)
        };

        let (r, g, b) = (
            (color.0 * 255.0).round() as u8,
            (color.1 * 255.0).round() as u8,
            (color.2 * 255.0).round() as u8
        );

        [r, g, b, 0xFF]
    }
}

impl CanvasColor for Rgb {
    fn as_rgb(&self) -> Rgb {
        *self
    }
    fn alpha(&self) -> f32 {
        1.0
    }
}

impl CanvasColor for Rgba {
    fn as_rgb(&self) -> Rgb {
        self.0
    }
    fn alpha(&self) -> f32 {
        self.1
    }
}

macro_rules! float_impls {
    ($($ty:ty),*) => { $( 
        impl CanvasColor for ($ty, $ty, $ty) {
            fn as_rgb(&self) -> Rgb {
                Rgb(self.0 as f32, self.1 as f32, self.2 as f32)
            }
            fn alpha(&self) -> f32 {
                1.0
            }
        }

        impl CanvasColor for ($ty, $ty, $ty, $ty) {
            fn as_rgb(&self) -> Rgb {
                Rgb(self.0 as f32, self.1 as f32, self.2 as f32)
            }
            fn alpha(&self) -> f32 {
                self.3 as f32
            }
        }

        impl CanvasColor for [$ty; 3] {
            fn as_rgb(&self) -> Rgb {
                Rgb(self[0] as f32, self[1] as f32, self[2] as f32)
            }
            fn alpha(&self) -> f32 {
                1.0
            }
        }

        impl CanvasColor for [$ty; 4] {
            fn as_rgb(&self) -> Rgb {
                Rgb(self[0] as f32, self[1] as f32, self[2] as f32)
            }
            fn alpha(&self) -> f32 {
                self[3] as f32
            }
        }

    )* }
}

float_impls!(f32, f64);

macro_rules! int_impls {
    ($($ty:ty),*) => { $(
        impl CanvasColor for ($ty, $ty, $ty) {
            fn as_rgb(&self) -> Rgb {
                Rgb(self.0 as f32 / 256.0, 
                    self.1 as f32 / 256.0, 
                    self.2 as f32 / 256.0)
            }
            fn alpha(&self) -> f32 {
                1.0
            }
        }

        impl CanvasColor for ($ty, $ty, $ty, $ty) {
            fn as_rgb(&self) -> Rgb {
                Rgb(self.0 as f32 / 256.0,
                    self.1 as f32 / 256.0, 
                    self.2 as f32 / 256.0)
            }
            fn alpha(&self) -> f32 {
                self.3 as f32 / 256.0
            }
        }
    )* }
}

int_impls!(u8, u16, u32, u64, usize);

impl CanvasColor for [u8; 4] {
    fn as_rgb(&self) -> Rgb {
        (self[0], self[1], self[2]).as_rgb()
    }

    fn alpha(&self) -> f32 {
        self[3] as f32 / 256.0
    }

    fn into_gpu(self, _: Option<Rgb>) -> [u8; 4] {
        self
    }
}

impl CanvasColor for u32 {
    fn as_rgb(&self) -> Rgb {
        let color: [u8; 4] = unsafe {
            mem::transmute(*self)
        };

        (color[2], color[1], color[0]).as_rgb()
    }

    fn alpha(&self) -> f32 {
        1.0
    }
}

impl CanvasColor for u64 {
    fn as_rgb(&self) -> Rgb {
        let color: [u8; 8] = unsafe {
            mem::transmute(*self)
        };

        (color[2], color[1], color[0]).as_rgb()
    }

    fn alpha(&self) -> f32 {
        1.0
    }
}

impl CanvasColor for i32 {
    fn as_rgb(&self) -> Rgb {
        (*self as u32).as_rgb()
    }

    fn alpha(&self) -> f32 {
        1.0
    }
}

impl CanvasColor for i64 {
    fn as_rgb(&self) -> Rgb {
        (*self as u64).as_rgb()
    }

    fn alpha(&self) -> f32 {
        1.0
    }
}