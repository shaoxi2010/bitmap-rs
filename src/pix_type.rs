use std::mem::size_of;
use super::{BitMap, BitMapResult};

pub trait PixExt {
    type Target;
    fn blend(self) -> Self::Target;
    fn rgb(rgb:RGB) -> Self;
}

#[cfg(feature="minifb")]
pub trait Minifb where Self: Sized {
    type From;
    fn transform(source: &mut [Self::From]) -> &mut [Self];
    fn painter(source: &mut [Self::From], width: usize, height: usize, closure: impl FnOnce(BitMap<Self>)->BitMapResult<()>) -> BitMapResult<()>;
}

pub trait Painter where Self: Sized + PixExt + Copy {
    fn painter(source: &mut [u8], width: usize, height: usize, closure: impl FnOnce(BitMap<Self>)->BitMapResult<()>) -> BitMapResult<()> {
        let data = unsafe {
            std::slice::from_raw_parts_mut(source as *mut _ as *mut Self, source.len() / size_of::<Self>())
        };
        let bitmap = BitMap::new(data, width, height)?;
        closure(bitmap)?;
        Ok(())
    }
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

impl Painter for RGB565 {}

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

impl Painter for ARGB32 {}

#[cfg(feature="minifb")]
impl Minifb for ARGB32 {
    type From = u32;

    fn transform(source: &mut [Self::From]) -> &mut [Self] {
        unsafe { std::mem::transmute(source) }
    }

    fn painter(source: &mut [Self::From], width: usize, height: usize, closure: impl FnOnce(BitMap<Self>)->BitMapResult<()>) -> BitMapResult<()> {
        let bitmap = BitMap::new(Self::transform(source), width, height)?;
        closure(bitmap)?;
        Ok(())
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
pub const RED: RGB = RGB(255,0,0);
pub const GREEN: RGB = RGB(0,255,0);
pub const BLUE: RGB = RGB(0,0,255);