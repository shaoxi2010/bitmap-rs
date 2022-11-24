mod pix_type;
mod asc_fonts;
use std::{mem::size_of, cell::Cell, cmp::min};
use thiserror::Error;
pub use pix_type::*;
pub use asc_fonts::*;
use std::convert::From;

#[derive(Error, Debug)]
pub enum BitMapError {
    #[error("The BitMap buffer is {buffer} need {total}")]
    NotEnoughBuffer{
        buffer: usize,
        total: usize,
    },
    #[error("The BitMap Draw Ops overflow set x:{x} but width: {width}")]
    OverFlowX{
        x: usize,
        width: usize
    },
    #[error("The BitMap Draw Ops overflow set y:{y} but height: {height}")]
    OverFlowY{
        y: usize,
        height: usize
    }
}

pub type BitMapResult<T> = Result<T, BitMapError>;

pub struct Point {
    x:usize,
    y:usize,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

pub trait DrawIo {
    fn draw_pix(&self, topleft: Point, color: RGB) -> BitMapResult<()>;

    type PixType;
    fn bitblit(&self, topleft: Point, bitmap: &BitMap<Self::PixType>) -> BitMapResult<()>;

    fn draw_text(&self, topleft: Point, color: RGB, text: &str, size: usize) -> BitMapResult<()>;

}
pub struct BitMap<'a, T> {
    data: &'a [Cell<T>],
    width: usize,
    height: usize
}


impl<'a, T: PixExt+Copy> BitMap<'a, T> {
    pub fn new(data: &'a mut [T], width: usize, height: usize) -> BitMapResult<Self> {
        let data = Cell::from_mut(data).as_slice_of_cells();
        if data.len() < width * height {
            Err(BitMapError::NotEnoughBuffer { buffer: data.len() * size_of::<T>(), total: width * height * size_of::<T>() })
        } else {
            Ok(Self { data, width, height })
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x > self.width || y > self.height {
            None
        } else {
            let offset = self.width * y + x;
            Some(self.data[offset].get())
        }
    }

    pub fn set(&self, x: usize, y: usize, val: T) {
        if x > self.width || y > self.height {
            return;
        } else {
            let offset = self.width * y + x;
            self.data[offset].set(val)
        }
    }
}

impl<'a, T:PixExt+Copy> DrawIo for BitMap<'a, T> {
    fn draw_pix(&self, topleft: Point, color: RGB) -> BitMapResult<()> {
        let val = T::rgb(color);
        if topleft.x >= self.width {
            return Err(BitMapError::OverFlowX { x: topleft.x, width: self.width });
        }
        if topleft.y >= self.height {
            return Err(BitMapError::OverFlowY { y: topleft.y, height: self.height});
        }
        let offset = self.width * topleft.y + topleft.x;
        self.data[offset].set(val);
        Ok(())
    }

    type PixType = T;

    fn bitblit(&self, topleft: Point, bitmap: &BitMap<Self::PixType>) -> BitMapResult<()> {
        if topleft.x >= self.width {
            return Err(BitMapError::OverFlowX { x: topleft.x, width: self.width });
        }
        if topleft.y >= self.height {
            return Err(BitMapError::OverFlowY { y: topleft.y, height: self.height});
        }
        
        let xend = min(topleft.x + bitmap.width, self.width);
        let yend = min(topleft.y + bitmap.height, self.height);

        for i in topleft.x..xend {
            for j in topleft.y..yend {
                if let Some(val) = bitmap.get(i - topleft.x, j - topleft.y) {
                    self.set(i, j, val);
                }
            }
        }

        Ok(())
    }

    fn draw_text(&self, topleft: Point, color: RGB, text: &str, size: usize) -> BitMapResult<()> {
        if topleft.x >= self.width {
            return Err(BitMapError::OverFlowX { x: topleft.x, width: self.width });
        }
        if topleft.y >= self.height {
            return Err(BitMapError::OverFlowY { y: topleft.y, height: self.height});
        }
        let text_bitmaps = text.chars()
            .map(|char|char_bitmap(char, size, color))
            .collect::<Vec<_>>();
        let mut current_x = topleft.x;
        for mut bitmap in text_bitmaps {
            if self.bitblit((current_x, topleft.y).into(), &bitmap.bitmap()).is_err() {
                break;
            }
            current_x += bitmap.bitmap().width;
        }
        
        Ok(())
    }
}