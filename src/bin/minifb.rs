use minifb::{Key, Window, WindowOptions};
use bitmap::{ARGB32, DrawIo, Minifb, WHITE, BitMap, BitMapResult, PixExt, Point, GREEN};

const WIDTH: usize = 800;
const HEIGHT: usize = 480;

const PROGRESS_WIDTH:usize = 600;
const PROGRESS_HEIGHT:usize = 80;
const PROGRESS_BLOCK_SPARE:usize = 2;
const PROGRESS_BLOCKS:usize = 20;
const PROGRESS_BLOCK_HEIGHT:usize = PROGRESS_HEIGHT - 2 * PROGRESS_BLOCK_SPARE;
const PROGRESS_BLOCK_WIDTH:usize = PROGRESS_WIDTH / PROGRESS_BLOCKS - 2 * PROGRESS_BLOCK_SPARE;

fn draw_progress<T: PixExt + Copy + Default>(bitmap: BitMap<T>, val: usize, text: &str) -> BitMapResult<()> {
    use std::cmp::min;
    let textlen = 16 * text.len();
    bitmap.draw_text(((WIDTH - textlen) / 2,(HEIGHT - PROGRESS_HEIGHT) / 2 - 32 - PROGRESS_BLOCK_SPARE).into(), WHITE, text, 32)?;
    bitmap.fill_rectagle(((WIDTH - PROGRESS_WIDTH) / 2, (HEIGHT - PROGRESS_HEIGHT) / 2).into(), PROGRESS_WIDTH, PROGRESS_HEIGHT, WHITE)?; 
    let val = min(100, val);
    for x in 0..val / (100 / PROGRESS_BLOCKS) {
        let block_topleft:Point = ((WIDTH - PROGRESS_WIDTH) / 2 + PROGRESS_BLOCK_SPARE + PROGRESS_WIDTH / PROGRESS_BLOCKS * x,
         (HEIGHT - PROGRESS_HEIGHT) / 2 + PROGRESS_BLOCK_SPARE).into();
         bitmap.fill_rectagle(block_topleft, PROGRESS_BLOCK_WIDTH, PROGRESS_BLOCK_HEIGHT, GREEN)?;
    }
    Ok(())
}

fn main() {
    let mut buffer = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut x = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ARGB32::painter(&mut buffer, WIDTH, HEIGHT, |bitmap| {
            draw_progress(bitmap, x / 10, &format!("Progress:{}%", std::cmp::min(100, x/10)))
        }).unwrap();
        x += 1;
        if x > 1050 {
            x = 0;
            ARGB32::painter(&mut buffer, WIDTH, HEIGHT, |bitmap| {
                bitmap.clear()
            }).unwrap()
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}