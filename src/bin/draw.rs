use bitmap::{BitMap, ARGB32, DrawIo, RGB, PixExt};

fn main() {
    let mut data = vec![ARGB32::default(); 32*32];
    let bitmap = BitMap::new(&mut data, 32, 32).unwrap();
    bitmap.draw_pix(0, 0, RGB::new(0, 255, 0)).unwrap();
    bitmap.draw_pix(31, 31, RGB::new(0, 0, 255)).unwrap();

    let mut data2 = vec![ARGB32::default(); 64*64];
    let bitmap2 = BitMap::new(&mut data2, 64, 64).unwrap();
    bitmap2.bitblit(12, 12, &bitmap).unwrap();

    let raw = data2.into_iter().map(|x| x.blend()).collect::<Vec<_>>();
    
    println!("{:?}", raw);
}