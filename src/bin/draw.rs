use bitmap::{BitMap, ARGB32, DrawIo, WHITE, PixExt, FONT_ASC16};

fn main() {
    let mut char = FONT_ASC16.char_bitmap('c', WHITE);
    let mut data2 = vec![ARGB32::default(); 16*16];
    let bitmap2 = BitMap::new(&mut data2, 16, 16).unwrap();
    bitmap2.bitblit(0, 0, &char.bitmap()).unwrap();

    let raw = data2.into_iter().map(|x| x.blend()).collect::<Vec<_>>();
    
    println!("{:?}", raw);
}