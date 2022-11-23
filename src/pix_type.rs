pub trait PixExt {
    type Target;
    fn blend(self) -> Self::Target;
    fn rgb(rgb:RGB) -> Self;
}

#[derive(Clone, Copy, Default)]
pub struct RGB565(u16);

impl PixExt for RGB565 {

    type Target = u16;

    fn blend(self) -> Self::Target {
        self.0
    }

    fn rgb(rgb: RGB) -> Self {
        Self(((rgb.0 as u16 & 0b11111000) << 8) | ((rgb.1 as u16 & 0b11111100) << 3) | (rgb.2 as u16 >> 3))
    }
}

#[derive(Clone, Copy, Default)]
pub struct ARGB32(u32);
impl PixExt for ARGB32 {
    type Target = u32;

    fn blend(self) -> Self::Target {
        self.0
    }

    fn rgb(rgb:RGB) -> Self {
        Self(0xff << 24 | (rgb.0 as u32) << 16 | (rgb.1 as u32) << 8 | rgb.2 as u32)
    }
}

#[derive(Clone, Copy)]
pub struct RGB(u8, u8, u8);
impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }
}

pub const BLACK: RGB = RGB(0, 0, 0);
pub const WHITE: RGB = RGB(255,255,255);