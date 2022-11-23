use super::pix_type::{PixExt, RGB, BLACK};
use super::BitMap;
use bitter::{LittleEndianReader, BitReader};
pub struct AsciiFont {
    data: &'static [u8],
    len: usize, 
}

impl AsciiFont {
    fn char_bytes(&self) -> usize {
        self.len * self.len / 8
    }

    fn map_to_data<T: PixExt+Copy>(&self, index: char, rgb:RGB) -> Vec<T> {
        let mut data = Vec::new();
        let offset = index as usize * self.char_bytes();
        let mut bitreader = LittleEndianReader::new(&self.data[offset..offset+self.char_bytes()]);
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
            len: self.len,
        }
    }
}

pub struct CharBitMap<T> {
    data: Vec<T>,
    len: usize,
}

impl<T: PixExt+Copy> CharBitMap<T> {
    pub fn bitmap(&mut self) -> BitMap<T> {
        BitMap::new(&mut self.data, self.len, self.len).unwrap()
    }
}

pub const FONT_ASC12: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC12"), len: 12};
pub const FONT_ASC16: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC16"), len: 16};
pub const FONT_ASC48: AsciiFont = AsciiFont {data: include_bytes!("fonts/ASC48"), len: 48};