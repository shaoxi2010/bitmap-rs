mod pix_type;
mod asc_fonts;
use std::{mem::size_of, cell::Cell, cmp::min};
use thiserror::Error;
pub use pix_type::*;
pub use asc_fonts::*;
use std::convert::From;

const KSCAL:isize = 100;

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

#[derive(Clone, Copy)]
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
    type PixType;
    fn bitblit(&self, topleft: Point, bitmap: &BitMap<Self::PixType>) -> BitMapResult<()>;
    fn draw_pix(&self, topleft: Point, color: RGB) -> BitMapResult<()>;
    fn draw_text(&self, topleft: Point, color: RGB, text: &str, size: usize) -> BitMapResult<()>;
    fn draw_line(&self, start: Point, end: Point, color: RGB) -> BitMapResult<()>;
    fn draw_rectagle(&self, topleft: Point, width: usize, height:usize, color: RGB) -> BitMapResult<()>;
    fn fill_rectagle(&self, topleft: Point, width: usize, height:usize, color: RGB) -> BitMapResult<()>;
    fn clear(&self) ->BitMapResult<()>;
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

impl<'a, T:PixExt+Copy+Default> DrawIo for BitMap<'a, T> {
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

    fn draw_line(&self, start: Point, end: Point, color: RGB) -> BitMapResult<()> {
        let start: Point = (min(start.x, self.width - 1), min(start.y, self.height - 1)).into();
        let end: Point = (min(end.x, self.width - 1), min(end.y, self.height - 1)).into();
        let deltax = end.x as isize - start.x as isize;
        let deltay = end.y as isize - start.y as isize;

        if deltax == 0 {
            let x = start.x;
            let y = min(start.y, end.y);
            for y in y..y + deltay.abs() as usize {
                self.draw_pix((x,y).into(), color)?;
            }
        } else if deltay == 0 {
            let y = start.y;
            let x = min(start.x, end.x);
            for x in x..x + deltax.abs() as usize {
                self.draw_pix((x,y).into(), color)?;
            }
        } else if deltax.abs() > deltay.abs() {
            let delta = deltax.abs();
            let k = deltay * KSCAL / deltax;
            let xstep = deltax / delta;
            for i in 0..delta {
                let x = start.x as isize + xstep * i; 
                let y = k * i * xstep / KSCAL+ start.y as isize;
                self.draw_pix((x as usize, y as usize).into(), color)?; 
            }
        } else {
            let delta = deltay.abs();
            let k = deltax * KSCAL / deltay;
            let ystep = deltay / delta;
            for i in 0..delta {
                let y = start.y as isize + ystep * i;
                let x = k * i * ystep / KSCAL + start.x as isize;
                self.draw_pix((x as usize, y as usize).into(), color)?; 
            }
        }
        Ok(())
    }

    fn draw_rectagle(&self, topleft: Point, width: usize, height:usize, color: RGB) -> BitMapResult<()> {
        let topright = (topleft.x + width, topleft.y);
        let bottomleft = (topleft.x , topleft.y + height);
        let bottomright = (topleft.x +  width, topleft.y + height);

        self.draw_line(topleft, topright.into(), color)?;
        self.draw_line(topleft, bottomleft.into(), color)?;
        self.draw_line(bottomright.into(), topright.into(), color)?;
        self.draw_line(bottomright.into(), bottomleft.into(), color)?;
        Ok(())
    }

    fn fill_rectagle(&self, topleft: Point, width: usize, height:usize, color: RGB) -> BitMapResult<()> {
        for y in topleft.y..=topleft.y+height {
            let line_start: Point = (topleft.x, y).into();   
            let line_end: Point = (topleft.x + width, y).into();
            self.draw_line(line_start, line_end, color)?;
        }

        Ok(())
    }

    fn clear(&self) ->BitMapResult<()> {
        for pix in self.data {
            pix.set(T::default());
        }
        Ok(())
    }
}