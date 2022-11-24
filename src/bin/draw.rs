use bitmap::{char_bitmap, BitMap, DrawIo, PixExt, ARGB32, WHITE};

fn main() {
    let mut char = char_bitmap('c', 16, WHITE);
    let mut data2 = vec![ARGB32::default(); 16 * 16];
    let bitmap2 = BitMap::new(&mut data2, 16, 16).unwrap();
    bitmap2.bitblit((0, 0).into(), &char.bitmap()).unwrap();

    let raw = data2.into_iter().map(|x| x.blend()).collect::<Vec<_>>();

    println!("{:?}", raw);
}
