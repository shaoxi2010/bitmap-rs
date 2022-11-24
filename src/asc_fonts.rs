use super::pix_type::{PixExt, RGB, BLACK};
use super::BitMap;
use bitter::{BigEndianReader, BitReader};
pub struct AsciiFont {
    data: &'static [u8],
    width: usize, 
    height: usize,
}

impl AsciiFont {
    fn char_bytes(&self) -> usize {
        self.width * self.height / 8
    }

    fn map_to_data<T: PixExt+Copy>(&self, index: char, rgb:RGB) -> Vec<T> {
        let mut data = Vec::new();
        let offset = index as usize * self.char_bytes();
        let mut bitreader = BigEndianReader::new(&self.data[offset..offset+self.char_bytes()]);
        while let Some(bit) = bitreader.read_bit() {
            if bit {
                data.push(T::rgb(rgb));
            } else {
                data.push(T::rgb(BLACK));
            }
        }
        data
    }

    pub fn char_bitmap<T: PixExt+Copy>(&self, index: char, rgb:RGB) -> CharBitMap<T> {
        let char_data = self.map_to_data(index, rgb);
        CharBitMap {
            data: char_data,
            width: self.width,
            height: self.height
        }
    }
}

pub struct CharBitMap<T> {
    data: Vec<T>,
    width: usize, 
    height: usize,
}

impl<T: PixExt+Copy> CharBitMap<T> {
    pub fn bitmap(&mut self) -> BitMap<T> {
        BitMap::new(&mut self.data, self.width, self.height ).unwrap()
    }
}

pub const FONT_ASC12: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC12"), width: 8, height: 12};
pub const FONT_ASC16: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC16"), width:8, height:16};
pub const FONT_ASC32: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC32"), width:16, height:32};
pub const FONT_ASC48: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC48"), width:24, height:48};