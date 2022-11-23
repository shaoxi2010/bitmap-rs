mod pix_type;
use std::{mem::size_of, cell::Cell, cmp::min};
use thiserror::Error;
pub use pix_type::*;

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

pub trait DrawIo {
    fn draw_pix(&self, x:usize, y:usize, color: RGB) -> BitMapResult<()>;

    type PixType;
    fn bitblit(&self, x:usize, y:usize, bitmap: &BitMap<Self::PixType>) -> BitMapResult<()>;
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
    fn draw_pix(&self, x:usize, y:usize, color: RGB) -> BitMapResult<()> {
        let val = T::rgb(color);
        if x >= self.width {
            return Err(BitMapError::OverFlowX { x: x, width: self.width });
        }
        if y >= self.height {
            return Err(BitMapError::OverFlowY { y: y, height: self.height});
        }
        let offset = self.width * y + x;
        self.data[offset].set(val);
        Ok(())
    }

    type PixType = T;

    fn bitblit(&self, x:usize, y:usize, bitmap: &BitMap<Self::PixType>) -> BitMapResult<()> {
        if x >= self.width {
            return Err(BitMapError::OverFlowX { x: x, width: self.width });
        }
        if y >= self.height {
            return Err(BitMapError::OverFlowY { y: y, height: self.height});
        }
        
        let xend = min(x + bitmap.width, self.width);
        let yend = min(y + bitmap.height, self.height);

        for i in x..xend {
            for j in y..yend {
                println!("get{} {}", i, j);
                if let Some(val) = bitmap.get(i - x, j - y) {
                    self.set(i, j, val);
                }
            }
        }

        Ok(())
    }
}