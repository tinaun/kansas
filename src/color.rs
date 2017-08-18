use std::mem;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
/// internal representation of a color value
///
pub struct Rgb(f32, f32, f32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Rgba(Rgb, f32);

impl Rgb {
    pub fn blend_alpha(self, a: f32) -> Self {
        Rgb(self.0 * a, self.1 * a, self.2 * a)
    }

    // mix two colors together with a certain weight
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